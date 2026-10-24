mod byteorder;
mod bmp {

	use std::convert::AsRef;
	use std::fmt;
	use std::fs;
	use std::io;
	use std::io::{Cursor, Read, Write};
	use std::iter::Iterator;
	use std::path::Path;

	pub use decoder::{BmpError, BmpErrorKind, BmpResult};

	#[macro_export]
	macro_rules! px {
		($r:expr, $g:expr, $b:expr) => {
			Pixel {
				r: $r as u8,
				g: $g as u8,
				b: $b as u8,
			}
		};
	}

	macro_rules! file_size {
		($bpp:expr, $width:expr, $height:expr) => {{
			let header_size = 2 + 12 + 40;
			let row_size = (($bpp as f32 * $width as f32 + 31.0) / 32.0).floor() as u32 * 4;
			(header_size as u32, $height as u32 * row_size)
		}};
	}

	mod decoder {
		use super::super::byteorder;

		use byteorder::{LittleEndian, ReadBytesExt};

		use std::convert::{AsRef, From};
		use std::error::Error;
		use std::fmt;
		use std::io::{self, Cursor, Read, Seek, SeekFrom};

		const BMP_HEADER_SIZE: u64 = 14;

		use self::BmpErrorKind::*;
		use super::*;

		pub type BmpResult<T> = Result<T, BmpError>;

		#[derive(Debug)]
		pub struct BmpError {
			pub kind: BmpErrorKind,
			pub details: String,
		}

		impl BmpError {
			fn new<T: AsRef<str>>(
				kind: BmpErrorKind,
				details: T,
			) -> BmpError {
				BmpError {
					kind,
					details: String::from(details.as_ref()),
				}
			}
		}

		impl fmt::Display for BmpError {
			fn fmt(
				&self,
				fmt: &mut fmt::Formatter,
			) -> fmt::Result {
				match self.kind {
					BmpIoError(ref error) => error.fmt(fmt),
					ref e => {
						let kind_desc: &str = e.as_ref();
						write!(fmt, "{}: {}", kind_desc, self.details)
					}
				}
			}
		}

		impl From<io::Error> for BmpError {
			fn from(err: io::Error) -> BmpError {
				BmpError::new(BmpIoError(err), "Io Error")
			}
		}

		impl Error for BmpError {}

		#[derive(Debug)]
		pub enum BmpErrorKind {
			WrongMagicNumbers,
			UnsupportedBitsPerPixel,
			UnsupportedCompressionType,
			UnsupportedBmpVersion,
			UnsupportedHeader,
			BmpIoError(io::Error),
		}

		impl AsRef<str> for BmpErrorKind {
			fn as_ref(&self) -> &str {
				match *self {
					WrongMagicNumbers => "Wrong magic numbers",
					UnsupportedBitsPerPixel => "Unsupported bits per pixel",
					UnsupportedCompressionType => "Unsupported compression type",
					UnsupportedBmpVersion => "Unsupported BMP version",
					_ => "BMP Error",
				}
			}
		}

		pub fn decode_image(bmp_data: &mut Cursor<Vec<u8>>) -> BmpResult<Image> {
			read_bmp_id(bmp_data)?;
			let header = read_bmp_header(bmp_data)?;
			let dib_header = read_bmp_dib_header(bmp_data)?;

			let color_palette = read_color_palette(bmp_data, &dib_header)?;

			let width = dib_header
				.width
				.abs() as u32;
			let height = dib_header
				.height
				.abs() as u32;
			let padding = width % 4;

			let data = match color_palette {
				Some(ref palette) => read_indexes(
					bmp_data.get_mut(),
					&palette,
					width as usize,
					height as usize,
					dib_header.bits_per_pixel,
					header.pixel_offset as usize,
				)?,
				None => read_pixels(bmp_data, width, height, header.pixel_offset, padding as i64)?,
			};

			let image = Image {
				header,
				dib_header: BmpDibHeader::new(width as i32, height as i32),
				color_palette,
				width,
				height,
				padding,
				data,
			};

			Ok(image)
		}

		fn read_bmp_id(bmp_data: &mut Cursor<Vec<u8>>) -> BmpResult<()> {
			let mut bm = [0, 0];
			bmp_data.read(&mut bm)?;

			if bm == b"BM"[..] {
				Ok(())
			} else {
				Err(BmpError::new(
					WrongMagicNumbers,
					format!("Expected [66, 77], but was {:?}", bm),
				))
			}
		}

		fn read_bmp_header(bmp_data: &mut Cursor<Vec<u8>>) -> BmpResult<BmpHeader> {
			let header = BmpHeader {
				file_size: bmp_data.read_u32::<LittleEndian>()?,
				creator1: bmp_data.read_u16::<LittleEndian>()?,
				creator2: bmp_data.read_u16::<LittleEndian>()?,
				pixel_offset: bmp_data.read_u32::<LittleEndian>()?,
			};

			Ok(header)
		}

		fn read_bmp_dib_header(bmp_data: &mut Cursor<Vec<u8>>) -> BmpResult<BmpDibHeader> {
			let dib_header = BmpDibHeader {
				header_size: bmp_data.read_u32::<LittleEndian>()?,
				width: bmp_data.read_i32::<LittleEndian>()?,
				height: bmp_data.read_i32::<LittleEndian>()?,
				num_planes: bmp_data.read_u16::<LittleEndian>()?,
				bits_per_pixel: bmp_data.read_u16::<LittleEndian>()?,
				compress_type: bmp_data.read_u32::<LittleEndian>()?,
				data_size: bmp_data.read_u32::<LittleEndian>()?,
				hres: bmp_data.read_i32::<LittleEndian>()?,
				vres: bmp_data.read_i32::<LittleEndian>()?,
				num_colors: bmp_data.read_u32::<LittleEndian>()?,
				num_imp_colors: bmp_data.read_u32::<LittleEndian>()?,
			};

			match BmpVersion::from_dib_header(&dib_header) {
				Some(BmpVersion::Three) | Some(BmpVersion::Four) | Some(BmpVersion::Five) => (),
				Some(other) => return Err(BmpError::new(UnsupportedBmpVersion, other)),
				None => {
					return Err(BmpError::new(
						UnsupportedHeader,
						format!(
							"Only simple BMP images of version 3, 4, and 5 are currently supported. \
                Connot decode the image for the following header: {:?}",
							dib_header
						),
					));
				}
			}

			match dib_header.bits_per_pixel {
				1 | 4 | 8 | 24 => (),
				other => {
					return Err(BmpError::new(
						UnsupportedBitsPerPixel,
						format!(
							"Only 1, 4, 8, and 24 bits per pixel are currently supported, was: {}",
							other
						),
					));
				}
			}

			match CompressionType::from_u32(dib_header.compress_type) {
				CompressionType::Uncompressed => (),
				other => return Err(BmpError::new(UnsupportedCompressionType, other)),
			}

			Ok(dib_header)
		}

		fn read_color_palette(
			bmp_data: &mut Cursor<Vec<u8>>,
			dh: &BmpDibHeader,
		) -> BmpResult<Option<Vec<Pixel>>> {
			let num_entries = match dh.bits_per_pixel {
				_ if dh.num_colors != 0 => dh.num_colors as usize,
				bpp @ 1 | bpp @ 4 | bpp @ 8 => 1 << bpp,
				_ => return Ok(None),
			};

			let num_bytes = match BmpVersion::from_dib_header(&dh) {
				Some(BmpVersion::Two) => return Err(BmpError::new(UnsupportedBmpVersion, BmpVersion::Two)),
				_ => 4,
			};

			bmp_data.seek(SeekFrom::Start(BMP_HEADER_SIZE + dh.header_size as u64))?;

			let mut px = &mut [0; 4][0..num_bytes as usize];
			let mut color_palette = Vec::with_capacity(num_entries);
			for _ in 0..num_entries {
				bmp_data.read(&mut px)?;
				color_palette.push(px!(px[2], px[1], px[0]));
			}

			Ok(Some(color_palette))
		}

		fn read_indexes(
			bmp_data: &mut Vec<u8>,
			palette: &Vec<Pixel>,
			width: usize,
			height: usize,
			bpp: u16,
			offset: usize,
		) -> BmpResult<Vec<Pixel>> {
			let mut data = Vec::with_capacity(height * width);
			let bytes_per_row = (width as f64 / (8.0 / bpp as f64)).ceil() as usize;
			for y in 0..height {
				let padding = match bytes_per_row % 4 {
					0 => 0,
					other => 4 - other,
				};
				let start = offset + (bytes_per_row + padding) * y;
				let bytes = &bmp_data[start..start + bytes_per_row];

				for i in bit_index(&bytes, bpp as usize, width as usize) {
					data.push(palette[i]);
				}
			}
			Ok(data)
		}

		fn read_pixels(
			bmp_data: &mut Cursor<Vec<u8>>,
			width: u32,
			height: u32,
			offset: u32,
			padding: i64,
		) -> BmpResult<Vec<Pixel>> {
			let mut data = Vec::with_capacity((height * width) as usize);
			bmp_data.seek(SeekFrom::Start(offset as u64))?;
			let mut px = [0; 3];
			for _ in 0..height {
				for _ in 0..width {
					bmp_data.read(&mut px)?;
					data.push(px!(px[2], px[1], px[0]));
				}
				bmp_data.seek(SeekFrom::Current(padding))?;
			}
			Ok(data)
		}

		const BITS: usize = 8;

		#[derive(Debug)]
		struct BitIndex<'a> {
			size: usize,
			nbits: usize,
			bits_left: usize,
			mask: u8,
			bytes: &'a [u8],
			index: usize,
		}

		fn bit_index<'a>(
			bytes: &'a [u8],
			nbits: usize,
			size: usize,
		) -> BitIndex<'a> {
			let bits_left = BITS - nbits;
			BitIndex {
				size,
				nbits,
				bits_left,
				mask: (!0 as u8 >> bits_left),
				bytes,
				index: 0,
			}
		}

		impl<'a> Iterator for BitIndex<'a> {
			type Item = usize;

			fn next(&mut self) -> Option<usize> {
				let n = self.index / BITS;
				let offset = self.bits_left - self.index % BITS;

				self.index += self.nbits;

				if self.size == 0 {
					None
				} else {
					self.size -= 1;
					self
						.bytes
						.get(n)
						.map(|&block| ((block & self.mask << offset) >> offset) as usize)
				}
			}
		}
	}
	mod encoder {
		use super::super::byteorder::{LittleEndian, WriteBytesExt};

		use std::io::{self, Write};

		use super::Image;

		const B: u8 = 66;
		const M: u8 = 77;

		pub fn encode_image(bmp_image: &Image) -> io::Result<Vec<u8>> {
			let mut bmp_data = Vec::with_capacity(
				bmp_image
					.header
					.file_size as usize,
			);

			write_header(&mut bmp_data, bmp_image)?;
			write_data(&mut bmp_data, bmp_image)?;
			Ok(bmp_data)
		}

		fn write_header(
			bmp_data: &mut Vec<u8>,
			img: &Image,
		) -> io::Result<()> {
			let header = &img.header;
			let dib_header = &img.dib_header;
			let (header_size, data_size) = file_size!(24, img.width, img.height);

			io::Write::write(bmp_data, &[B, M])?;

			bmp_data.write_u32::<LittleEndian>(header_size + data_size)?;
			bmp_data.write_u16::<LittleEndian>(header.creator1)?;
			bmp_data.write_u16::<LittleEndian>(header.creator2)?;
			bmp_data.write_u32::<LittleEndian>(header_size)?;
			bmp_data.write_u32::<LittleEndian>(dib_header.header_size)?;
			bmp_data.write_i32::<LittleEndian>(dib_header.width)?;
			bmp_data.write_i32::<LittleEndian>(dib_header.height)?;
			bmp_data.write_u16::<LittleEndian>(1)?;
			bmp_data.write_u16::<LittleEndian>(24)?;
			bmp_data.write_u32::<LittleEndian>(0)?;
			bmp_data.write_u32::<LittleEndian>(data_size)?;
			bmp_data.write_i32::<LittleEndian>(dib_header.hres)?;
			bmp_data.write_i32::<LittleEndian>(dib_header.vres)?;
			bmp_data.write_u32::<LittleEndian>(0)?;
			bmp_data.write_u32::<LittleEndian>(0)?;
			Ok(())
		}

		fn write_data(
			bmp_data: &mut Vec<u8>,
			img: &Image,
		) -> io::Result<()> {
			let padding = &[0; 4][0..img.padding as usize];
			for y in 0..img.height {
				for x in 0..img.width {
					let index = (y * img.width + x) as usize;
					let px = &img.data[index];
					Write::write(bmp_data, &[px.b, px.g, px.r])?;
				}
				Write::write(bmp_data, padding)?;
			}
			Ok(())
		}
	}

	#[derive(Clone, Copy, Debug, Eq, PartialEq)]
	#[repr(C)]
	pub struct Pixel {
		pub r: u8,
		pub g: u8,
		pub b: u8,
	}

	impl Pixel {
		pub fn new(
			r: u8,
			g: u8,
			b: u8,
		) -> Pixel {
			Pixel { r, g, b }
		}
	}

	impl fmt::Display for Pixel {
		fn fmt(
			&self,
			f: &mut fmt::Formatter,
		) -> fmt::Result {
			write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
		}
	}

	impl fmt::UpperHex for Pixel {
		fn fmt(
			&self,
			f: &mut fmt::Formatter,
		) -> fmt::Result {
			write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
		}
	}

	impl fmt::LowerHex for Pixel {
		fn fmt(
			&self,
			f: &mut fmt::Formatter,
		) -> fmt::Result {
			write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
		}
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	enum BmpVersion {
		Two,
		Three,
		ThreeNT,
		Four,
		Five,
	}

	impl BmpVersion {
		fn from_dib_header(dib_header: &BmpDibHeader) -> Option<BmpVersion> {
			match dib_header.header_size {
				12 => Some(BmpVersion::Two),
				40 if dib_header.compress_type == 3 => Some(BmpVersion::ThreeNT),
				40 => Some(BmpVersion::Three),
				108 => Some(BmpVersion::Four),
				124 => Some(BmpVersion::Five),
				_ => None,
			}
		}
	}

	impl AsRef<str> for BmpVersion {
		fn as_ref(&self) -> &str {
			match *self {
				BmpVersion::Two => "BMP Version 2",
				BmpVersion::Three => "BMP Version 3",
				BmpVersion::ThreeNT => "BMP Version 3 NT",
				BmpVersion::Four => "BMP Version 4",
				BmpVersion::Five => "BMP Version 5",
			}
		}
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	enum CompressionType {
		Uncompressed,
		Rle8bit,
		Rle4bit,
		BitfieldsEncoding,
	}

	impl CompressionType {
		fn from_u32(val: u32) -> CompressionType {
			match val {
				1 => CompressionType::Rle8bit,
				2 => CompressionType::Rle4bit,
				3 => CompressionType::BitfieldsEncoding,
				_ => CompressionType::Uncompressed,
			}
		}
	}

	impl AsRef<str> for CompressionType {
		fn as_ref(&self) -> &str {
			match *self {
				CompressionType::Rle8bit => "RLE 8-bit",
				CompressionType::Rle4bit => "RLE 4-bit",
				CompressionType::BitfieldsEncoding => "Bitfields Encoding",
				CompressionType::Uncompressed => "Uncompressed",
			}
		}
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	pub struct BmpHeader {
		file_size: u32,
		creator1: u16,
		creator2: u16,
		pixel_offset: u32,
	}

	impl BmpHeader {
		fn new(
			header_size: u32,
			data_size: u32,
		) -> BmpHeader {
			BmpHeader {
				file_size: header_size + data_size,
				creator1: 0,
				creator2: 0,
				pixel_offset: header_size,
			}
		}
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	pub struct BmpDibHeader {
		header_size: u32,
		width: i32,
		height: i32,
		num_planes: u16,
		bits_per_pixel: u16,
		compress_type: u32,
		data_size: u32,
		hres: i32,
		vres: i32,
		num_colors: u32,
		num_imp_colors: u32,
	}

	impl BmpDibHeader {
		fn new(
			width: i32,
			height: i32,
		) -> BmpDibHeader {
			let (_, pixel_array_size) = file_size!(24, width, height);
			BmpDibHeader {
				header_size: 40,
				width,
				height,
				num_planes: 1,
				bits_per_pixel: 24,
				compress_type: 0,
				data_size: pixel_array_size,
				hres: 1000,
				vres: 1000,
				num_colors: 0,
				num_imp_colors: 0,
			}
		}
	}

	#[derive(Clone, Eq, PartialEq)]
	pub struct Image {
		pub header: BmpHeader,
		pub dib_header: BmpDibHeader,
		pub color_palette: Option<Vec<Pixel>>,
		pub width: u32,
		pub height: u32,
		pub padding: u32,
		pub data: Vec<Pixel>,
	}

	impl Image {
		pub fn new(
			width: u32,
			height: u32,
		) -> Image {
			let mut data = Vec::with_capacity((width * height) as usize);
			for _ in 0..width * height {
				data.push(px!(0, 0, 0));
			}

			let (header_size, data_size) = file_size!(24, width, height);
			Image {
				header: BmpHeader::new(header_size, data_size),
				dib_header: BmpDibHeader::new(width as i32, height as i32),
				color_palette: None,
				width,
				height,
				padding: width % 4,
				data,
			}
		}

		#[inline]
		pub fn get_width(&self) -> u32 {
			self.width
		}

		#[inline]
		pub fn get_height(&self) -> u32 {
			self.height
		}

		#[inline]
		pub fn set_pixel(
			&mut self,
			x: u32,
			y: u32,
			val: Pixel,
		) -> bool {
			self
				.data
				.get_mut(((self.height - y - 1) * self.width + x) as usize)
				.map(|a| *a = val)
				.is_some()
		}

		#[inline]
		pub fn get_pixel(
			&self,
			x: u32,
			y: u32,
		) -> Option<&Pixel> {
			self
				.data
				.get(((self.height - y - 1) * self.width + x) as usize)
		}

		#[inline]
		pub fn coordinates(&self) -> ImageIndex {
			ImageIndex::new(self.width as u32, self.height as u32)
		}

		pub fn save<P: AsRef<Path>>(
			&self,
			path: P,
		) -> io::Result<()> {
			let mut bmp_file = fs::File::create(path)?;
			self.to_writer(&mut bmp_file)
		}

		pub fn to_writer<W: Write>(
			&self,
			destination: &mut W,
		) -> io::Result<()> {
			let bmp_data = encoder::encode_image(self)?;
			destination.write(&bmp_data)?;
			Ok(())
		}
	}

	impl fmt::Debug for Image {
		fn fmt(
			&self,
			f: &mut fmt::Formatter,
		) -> Result<(), fmt::Error> {
			f.debug_struct("Image")
				.field("header", &self.header)
				.field("dib_header", &self.dib_header)
				.field("color_palette", &self.color_palette)
				.field("width", &self.width)
				.field("height", &self.height)
				.field("padding", &self.padding)
				.finish()
		}
	}

	#[derive(Clone, Copy)]
	pub struct ImageIndex {
		width: u32,
		height: u32,
		x: u32,
		y: u32,
	}

	impl ImageIndex {
		fn new(
			width: u32,
			height: u32,
		) -> ImageIndex {
			ImageIndex {
				width,
				height,
				x: 0,
				y: 0,
			}
		}
	}

	impl Iterator for ImageIndex {
		type Item = (u32, u32);

		fn next(&mut self) -> Option<(u32, u32)> {
			if self.x < self.width && self.y < self.height {
				let this = Some((self.x, self.y));
				self.x += 1;
				if self.x == self.width {
					self.x = 0;
					self.y += 1;
				}
				this
			} else {
				None
			}
		}
	}

	pub fn open<P: AsRef<Path>>(path: P) -> BmpResult<Image> {
		let mut f = fs::File::open(path)?;
		from_reader(&mut f)
	}

	pub fn from_reader<R: Read>(source: &mut R) -> BmpResult<Image> {
		let mut bytes = Vec::new();
		source.read_to_end(&mut bytes)?;

		let mut bmp_data = Cursor::new(bytes);
		decoder::decode_image(&mut bmp_data)
	}
}

use std::{
	ffi::{CStr, CString, OsStr, c_char},
	os::unix::ffi::OsStrExt,
};

pub use crate::bmp::Pixel;

#[repr(C)]
pub struct BmpImage {
	ptr: *mut crate::bmp::Pixel,
	size: u32,
	width: u32,
	height: u32,
	header: [u8; 12],
	dib_header: [u8; 40],
}
#[repr(C)]
pub struct Margin {
	left: i32,
	top: i32,
	right: i32,
	bottom: i32,
}

fn transform(inp: crate::bmp::Image) -> BmpImage {
	let width = inp.get_width();
	let height = inp.get_height();
	let (ptr, size, _) = Vec::into_raw_parts(inp.data);
	unsafe {
		BmpImage {
			ptr,
			size: size as u32,
			width,
			height,
			header: std::mem::transmute(inp.header),
			dib_header: std::mem::transmute(inp.dib_header),
		}
	}
}
fn transform_back(inp: *mut BmpImage) -> crate::bmp::Image {
	unsafe {
		crate::bmp::Image {
			header: std::mem::transmute((*inp).header),
			dib_header: std::mem::transmute((*inp).dib_header),
			color_palette: None,
			width: (*inp).width,
			height: (*inp).height,
			padding: (*inp).width % 4,
			data: Vec::from_raw_parts((*inp).ptr, (*inp).size as usize, (*inp).size as usize),
		}
	}
}

#[repr(C)]
pub struct ResultBmpImage {
	tag: u8,
	ptr: *const c_char,
	img: *mut BmpImage,
}
#[repr(C)]
pub struct ResultPixel {
	data: Pixel,
	tag: u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn open_bmp(s: *const c_char) -> ResultBmpImage {
	use crate::bmp::BmpErrorKind::*;
	match bmp::open(std::path::Path::new(OsStr::from_bytes(
		unsafe { CStr::from_ptr(s) }.to_bytes(),
	))) {
		Ok(a) => {
			let mut b = transform(a);
			ResultBmpImage {
				img: &raw mut b,
				ptr: std::ptr::null(),
				tag: 0,
			}
		}
		Err(bmp::BmpError { kind, details: s }) => {
			let tag = match kind {
				WrongMagicNumbers => 1,
				BmpIoError(_) => 3,
				_ => 2,
			};
			let c_str = CString::new(s).expect("details does not contain null bytes!");
			ResultBmpImage {
				tag,
				ptr: c_str.as_ptr(),
				img: std::ptr::null_mut(),
			}
		}
	}
}
#[unsafe(no_mangle)]
pub extern "C" fn write_bmp(
	s: *const c_char,
	img: *mut BmpImage,
) -> *const c_char {
	match transform_back(img).save(std::path::Path::new(OsStr::from_bytes(
		unsafe { CStr::from_ptr(s) }.to_bytes(),
	))) {
		Err(b) => {
			let c_str = CString::new(b.to_string()).expect("error message does not contain null bytes!");
			c_str.as_ptr() //MAYBE NULL, need t ocheck dealocation rules from CString
		}
		_ => std::ptr::null(),
	}
}
#[unsafe(no_mangle)]
pub extern "C" fn index(
	img: *mut BmpImage,
	x: u32,
	y: u32,
) -> u32 {
	let a = &transform_back(img);
	if x < a.get_width() && y < a.get_height() {
		x + y * a.get_width()
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
	transform_back(img)
		.get_pixel(x, y)
		.map_or_else(
			|| ResultPixel {
				data: Pixel::new(0, 0, 0),
				tag: 1,
			},
			|a| ResultPixel { data: *a, tag: 0 },
		)
}
#[unsafe(no_mangle)]
pub extern "C" fn set_pixel(
	img: *mut BmpImage,
	x: u32,
	y: u32,
	color: Pixel,
) -> bool {
	transform_back(img).set_pixel(x, y, color)
}
#[unsafe(no_mangle)]
pub extern "C" fn merge(
	img1: *const BmpImage,
	img2: *const BmpImage,
	blend: f32,
) -> ResultBmpImage {
	todo!()
}
#[unsafe(no_mangle)]
pub extern "C" fn margin(
	img: *const BmpImage,
	marg: Margin,
	fill: Pixel,
) -> ResultBmpImage {
	todo!()
}
#[unsafe(no_mangle)]
pub extern "C" fn shrink(
	img: *const BmpImage,
	newWidth: u32,
	newHeight: u32,
) -> ResultBmpImage {
	todo!()
}
#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
