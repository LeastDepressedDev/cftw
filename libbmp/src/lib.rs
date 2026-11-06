mod bmp;
mod byteorder;

use std::{
	ffi::{CStr, CString, OsStr, c_char},
	os::unix::ffi::OsStrExt,
};

pub use crate::bmp::Pixel;

#[repr(C)]
pub struct Margin {
	left: i32,
	top: i32,
	right: i32,
	bottom: i32,
}

#[repr(C)]
pub struct BmpImage {
	ptr: *mut Pixel,
	size: u32,     // number of Pixels (== Vec::len)
	capacity: u32, // == Vec::capacity
	width: u32,
	height: u32,
	header: [u8; 12],
	dib_header: [u8; 40],
}

#[repr(C)]
pub struct ResultBmpImage {
	ptr: *const c_char,
	img: *mut BmpImage,
	tag: u8,
}

#[repr(C)]
pub struct ResultPixel {
	data: Pixel,
	tag: u8,
}

// =====================================================================
// Internal helpers
// =====================================================================

/// Take ownership of a `crate::bmp::Image` and turn it into a `BmpImage`.
/// The pixel buffer ownership is transferred to the caller, who must
/// eventually release it via `free_bmp`.
fn image_into_bmp(img: crate::bmp::Image) -> BmpImage {
	let crate::bmp::Image {
		header,
		dib_header,
		width,
		height,
		data,
		..
	} = img;

	let (ptr, size, capacity) = Vec::into_raw_parts(data);
	unsafe {
		BmpImage {
			ptr,
			size: size as u32,
			capacity: capacity as u32,
			width,
			height,
			header: std::mem::transmute(header),
			dib_header: std::mem::transmute(dib_header),
		}
	}
}

/// Borrow a `BmpImage` as a `crate::bmp::Image` **without** taking
/// ownership of the underlying buffer. Only valid while the returned
/// value is alive.
unsafe fn as_image(img: *const BmpImage) -> std::mem::ManuallyDrop<crate::bmp::Image> {
	let b = unsafe { &*img };
	std::mem::ManuallyDrop::new(crate::bmp::Image {
		header: unsafe { std::mem::transmute(b.header) },
		dib_header: unsafe { std::mem::transmute(b.dib_header) },
		color_palette: None,
		width: b.width,
		height: b.height,
		padding: b.width % 4, // TODO: proper BMP row padding formula
		data: unsafe { Vec::from_raw_parts(b.ptr, b.size as usize, b.capacity as usize) },
	})
}

unsafe fn pixels<'a>(img: *const BmpImage) -> &'a [Pixel] {
	let b = unsafe { &*img };
	unsafe { std::slice::from_raw_parts(b.ptr, b.size as usize) }
}

unsafe fn pixels_mut<'a>(img: *mut BmpImage) -> &'a mut [Pixel] {
	let b = unsafe { &*img };
	unsafe { std::slice::from_raw_parts_mut(b.ptr, b.size as usize) }
}

/// Allocate a `BmpImage` on the heap from an owned pixel buffer.
/// Takes `Vec::into_boxed_slice` so that `cap == len`, which lets us
/// deallocate safely later with `Vec::from_raw_parts(ptr, size, size)`.
fn ok_bmp(
	data: Vec<Pixel>,
	width: u32,
	height: u32,
	header: [u8; 12],
	dib_header: [u8; 40],
) -> ResultBmpImage {
	let (ptr, size, capacity) = Vec::into_raw_parts(data);

	ResultBmpImage {
		tag: 0,
		ptr: std::ptr::null(),
		img: Box::into_raw(Box::new(BmpImage {
			ptr,
			size: size as u32,
			capacity: capacity as u32,
			width,
			height,
			header,
			dib_header,
		})),
	}
}

fn err_bmp(
	msg: &str,
	tag: u8,
) -> ResultBmpImage {
	let c_str = CString::new(msg).unwrap_or_else(|_| CString::new("error").unwrap());
	ResultBmpImage {
		tag,
		ptr: CString::into_raw(c_str) as *const c_char,
		img: std::ptr::null_mut(),
	}
}

fn err_str(msg: &str) -> *const c_char {
	let c_str = CString::new(msg).unwrap_or_else(|_| CString::new("error").unwrap());
	CString::into_raw(c_str) as *const c_char
}

// =====================================================================
// Exported functions
// =====================================================================

#[unsafe(no_mangle)]
pub extern "C" fn open_bmp(s: *const c_char) -> ResultBmpImage {
	use crate::bmp::BmpErrorKind::*;
	if s.is_null() {
		return err_bmp("open_bmp: null path", 3);
	}
	let path_bytes = unsafe { CStr::from_ptr(s) }.to_bytes();
	match bmp::open(std::path::Path::new(OsStr::from_bytes(path_bytes))) {
		Ok(a) => {
			let img = image_into_bmp(a);
			ResultBmpImage {
				tag: 0,
				ptr: std::ptr::null(),
				img: Box::into_raw(Box::new(img)),
			}
		}
		Err(bmp::BmpError { kind, details: s }) => {
			let tag = match kind {
				WrongMagicNumbers => 1,
				BmpIoError(_) => 3,
				_ => 2,
			};
			err_bmp(&s, tag)
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn write_bmp(
	s: *const c_char,
	img: *mut BmpImage,
) -> *const c_char {
	if s.is_null() || img.is_null() {
		return err_str("write_bmp: null argument");
	}
	let path_bytes = unsafe { CStr::from_ptr(s) }.to_bytes();

	// ManuallyDrop prevents the Vec inside the temporary `Image` from
	// freeing the buffer owned by `BmpImage`.
	let res = unsafe {
		let image = as_image(img);
		image.save(std::path::Path::new(OsStr::from_bytes(path_bytes)))
	};

	match res {
		Ok(()) => std::ptr::null(),
		Err(e) => err_str(&e.to_string()),
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn index(
	img: *mut BmpImage,
	x: u32,
	y: u32,
) -> u32 {
	if img.is_null() {
		return u32::MAX;
	}
	let b = unsafe { &*img };
	if x < b.width && y < b.height {
		x + y * b.width
	} else {
		u32::MAX
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn read_pixel(
	img: *mut BmpImage,
	x: u32,
	y: u32,
) -> ResultPixel {
	let fail = || ResultPixel {
		data: Pixel::new(0, 0, 0),
		tag: 1,
	};
	if img.is_null() {
		return fail();
	}
	let idx = index(img, x, y);
	if idx == u32::MAX {
		return fail();
	}
	let slice = unsafe { pixels(img) };
	match slice
		.get(idx as usize)
		.copied()
	{
		Some(p) => ResultPixel { data: p, tag: 0 },
		None => fail(),
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn set_pixel(
	img: *mut BmpImage,
	x: u32,
	y: u32,
	color: Pixel,
) -> bool {
	if img.is_null() {
		return false;
	}
	let idx = index(img, x, y);
	if idx == u32::MAX {
		return false;
	}
	let slice = unsafe { pixels_mut(img) };
	let i = idx as usize;
	if i < slice.len() {
		slice[i] = color;
		true
	} else {
		false
	}
}

// ---------------------------------------------------------------------
// merge
// ---------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn merge(
	img1: *const BmpImage,
	img2: *const BmpImage,
	blend: f32,
) -> ResultBmpImage {
	if img1.is_null() || img2.is_null() {
		return err_bmp("merge: null argument", 2);
	}
	let b1 = unsafe { &*img1 };
	let b2 = unsafe { &*img2 };

	if b1.width != b2.width || b1.height != b2.height {
		return err_bmp("merge: images must have equal dimensions", 2);
	}

	let d1 = unsafe { pixels(img1) };
	let d2 = unsafe { pixels(img2) };
	let expected = (b1.width as usize).saturating_mul(b1.height as usize);

	if d1.len() != expected || d2.len() != expected {
		return err_bmp("merge: buffer size mismatch", 2);
	}

	// NaN-safe: clamp panics on NaN min/max, but NaN *value* yields 0 in cast.
	// Be explicit.
	let blend = if blend.is_nan() {
		0.0
	} else {
		blend.clamp(0.0, 1.0)
	};
	let inv = 1.0 - blend;

	let mut out = Vec::with_capacity(expected);
	for (a, b) in d1
		.iter()
		.zip(d2.iter())
	{
		out.push(Pixel::new(
			(a.r as f32 * inv + b.r as f32 * blend) as u8,
			(a.g as f32 * inv + b.g as f32 * blend) as u8,
			(a.b as f32 * inv + b.b as f32 * blend) as u8,
		));
	}

	ok_bmp(out, b1.width, b1.height, b1.header, b1.dib_header)
}

// ---------------------------------------------------------------------
// margin
// ---------------------------------------------------------------------
//
// Положительное значение = обрезка с этой стороны,
// отрицательное           = расширение.
//   { 0, -50, 20, 0 } => +50 строк сверху, -20 столбцов справа.
//
// new_left   = left
// new_top    = top
// new_right  = width  - right
// new_bottom = height - bottom

#[unsafe(no_mangle)]
pub extern "C" fn margin(
	img: *const BmpImage,
	marg: Margin,
	fill: Pixel,
) -> ResultBmpImage {
	if img.is_null() {
		return err_bmp("margin: null argument", 2);
	}
	let b = unsafe { &*img };
	let src = unsafe { pixels(img) };
	let expected = (b.width as usize).saturating_mul(b.height as usize);
	if src.len() != expected {
		return err_bmp("margin: buffer size mismatch", 2);
	}

	let new_left = marg.left as i64;
	let new_top = marg.top as i64;
	let new_right = b.width as i64 - marg.right as i64;
	let new_bottom = b.height as i64 - marg.bottom as i64;

	let new_w_i = new_right - new_left;
	let new_h_i = new_bottom - new_top;

	if new_w_i <= 0 || new_h_i <= 0 {
		return err_bmp("margin: empty resulting image", 2);
	}
	if new_w_i > u32::MAX as i64 || new_h_i > u32::MAX as i64 {
		return err_bmp("margin: resulting image too large", 2);
	}

	let new_w = new_w_i as u32;
	let new_h = new_h_i as u32;
	let Some(total) = (new_w as usize).checked_mul(new_h as usize) else {
		return err_bmp("margin: resulting image too large", 2);
	};

	let mut out = Vec::with_capacity(total);
	for ny in 0..new_h {
		for nx in 0..new_w {
			let ox = nx as i64 + new_left;
			let oy = ny as i64 + new_top;
			if ox >= 0 && oy >= 0 && ox < b.width as i64 && oy < b.height as i64 {
				out.push(src[(oy as usize) * (b.width as usize) + ox as usize]);
			} else {
				out.push(fill);
			}
		}
	}

	ok_bmp(out, new_w, new_h, b.header, b.dib_header)
}

// ---------------------------------------------------------------------
// shrink
// ---------------------------------------------------------------------
//
// Box-фильтр: каждый целевой пиксель — среднее по прямоугольнику
// исходных пикселей, попавших в его «ячейку».

#[unsafe(no_mangle)]
pub extern "C" fn shrink(
	img: *const BmpImage,
	new_width: u32,
	new_height: u32,
) -> ResultBmpImage {
	if img.is_null() {
		return err_bmp("shrink: null argument", 2);
	}
	if new_width == 0 || new_height == 0 {
		return err_bmp("shrink: dimensions must be positive", 2);
	}
	let b = unsafe { &*img };
	let src = unsafe { pixels(img) };
	let expected = (b.width as usize).saturating_mul(b.height as usize);
	if src.len() != expected {
		return err_bmp("shrink: buffer size mismatch", 2);
	}
	let Some(total) = (new_width as usize).checked_mul(new_height as usize) else {
		return err_bmp("shrink: dimensions too large", 2);
	};

	let mut out = Vec::with_capacity(total);
	for ny in 0..new_height {
		let sy0 = ((ny as u64 * b.height as u64) / new_height as u64) as u32;
		let mut sy1 =
			(((ny as u64 + 1) * b.height as u64 + new_height as u64 - 1) / new_height as u64) as u32;
		if sy1 <= sy0 {
			sy1 = sy0 + 1;
		}
		if sy1 > b.height {
			sy1 = b.height;
		}

		for nx in 0..new_width {
			let sx0 = ((nx as u64 * b.width as u64) / new_width as u64) as u32;
			let mut sx1 =
				(((nx as u64 + 1) * b.width as u64 + new_width as u64 - 1) / new_width as u64) as u32;
			if sx1 <= sx0 {
				sx1 = sx0 + 1;
			}
			if sx1 > b.width {
				sx1 = b.width;
			}

			let mut r_sum: u64 = 0;
			let mut g_sum: u64 = 0;
			let mut b_sum: u64 = 0;
			let mut count: u64 = 0;

			for sy in sy0..sy1 {
				let row = (sy as usize) * (b.width as usize);
				for sx in sx0..sx1 {
					let p = src[row + sx as usize];
					r_sum += p.r as u64;
					g_sum += p.g as u64;
					b_sum += p.b as u64;
					count += 1;
				}
			}

			// count != 0 гарантированно (см. клампы выше)
			out.push(Pixel::new(
				(r_sum / count) as u8,
				(g_sum / count) as u8,
				(b_sum / count) as u8,
			));
		}
	}

	ok_bmp(out, new_width, new_height, b.header, b.dib_header)
}

// =====================================================================
// Free functions
// =====================================================================

/// Free a `BmpImage` returned by `open_bmp` / `merge` / `margin` / `shrink`.
/// Passing a null pointer is a no-op. Passing any other pointer not
/// produced by this library is undefined behaviour.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_bmp(img: *mut BmpImage) {
	if img.is_null() {
		return;
	}
	unsafe {
		let boxed = Box::from_raw(img);
		// reclaim the pixel buffer
		drop(Vec::from_raw_parts(
			boxed.ptr,
			boxed.size as usize,
			boxed.capacity as usize,
		));
		// boxed is dropped here -> BmpImage struct freed
	}
}

/// Free a `\0`-terminated string returned by `write_bmp` or by the `ptr`
/// field of a failing `ResultBmpImage`. Null pointers are a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(s: *const c_char) {
	if s.is_null() {
		return;
	}
	unsafe {
		drop(CString::from_raw(s as *mut c_char));
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
