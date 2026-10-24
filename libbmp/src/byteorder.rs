#![allow(unused)]
use core::{
	convert::TryInto, fmt::Debug, hash::Hash, mem::align_of, ptr::copy_nonoverlapping, slice,
};

pub use io::{ReadBytesExt, WriteBytesExt};

mod io {
	use std::{
		io::{self, Result},
		slice,
	};

	use super::ByteOrder;

	pub trait ReadBytesExt: io::Read {
		#[inline]
		fn read_u8(&mut self) -> Result<u8> {
			let mut buf = [0; 1];
			self.read_exact(&mut buf)?;
			Ok(buf[0])
		}

		#[inline]
		fn read_i8(&mut self) -> Result<i8> {
			let mut buf = [0; 1];
			self.read_exact(&mut buf)?;
			Ok(buf[0] as i8)
		}

		#[inline]
		fn read_u16<T: ByteOrder>(&mut self) -> Result<u16> {
			let mut buf = [0; 2];
			self.read_exact(&mut buf)?;
			Ok(T::read_u16(&buf))
		}

		#[inline]
		fn read_i16<T: ByteOrder>(&mut self) -> Result<i16> {
			let mut buf = [0; 2];
			self.read_exact(&mut buf)?;
			Ok(T::read_i16(&buf))
		}

		#[inline]
		fn read_u24<T: ByteOrder>(&mut self) -> Result<u32> {
			let mut buf = [0; 3];
			self.read_exact(&mut buf)?;
			Ok(T::read_u24(&buf))
		}

		#[inline]
		fn read_i24<T: ByteOrder>(&mut self) -> Result<i32> {
			let mut buf = [0; 3];
			self.read_exact(&mut buf)?;
			Ok(T::read_i24(&buf))
		}

		#[inline]
		fn read_u32<T: ByteOrder>(&mut self) -> Result<u32> {
			let mut buf = [0; 4];
			self.read_exact(&mut buf)?;
			Ok(T::read_u32(&buf))
		}

		#[inline]
		fn read_i32<T: ByteOrder>(&mut self) -> Result<i32> {
			let mut buf = [0; 4];
			self.read_exact(&mut buf)?;
			Ok(T::read_i32(&buf))
		}

		#[inline]
		fn read_u48<T: ByteOrder>(&mut self) -> Result<u64> {
			let mut buf = [0; 6];
			self.read_exact(&mut buf)?;
			Ok(T::read_u48(&buf))
		}

		#[inline]
		fn read_i48<T: ByteOrder>(&mut self) -> Result<i64> {
			let mut buf = [0; 6];
			self.read_exact(&mut buf)?;
			Ok(T::read_i48(&buf))
		}

		#[inline]
		fn read_u64<T: ByteOrder>(&mut self) -> Result<u64> {
			let mut buf = [0; 8];
			self.read_exact(&mut buf)?;
			Ok(T::read_u64(&buf))
		}

		#[inline]
		fn read_i64<T: ByteOrder>(&mut self) -> Result<i64> {
			let mut buf = [0; 8];
			self.read_exact(&mut buf)?;
			Ok(T::read_i64(&buf))
		}

		#[inline]
		fn read_u128<T: ByteOrder>(&mut self) -> Result<u128> {
			let mut buf = [0; 16];
			self.read_exact(&mut buf)?;
			Ok(T::read_u128(&buf))
		}

		#[inline]
		fn read_i128<T: ByteOrder>(&mut self) -> Result<i128> {
			let mut buf = [0; 16];
			self.read_exact(&mut buf)?;
			Ok(T::read_i128(&buf))
		}

		#[inline]
		fn read_uint<T: ByteOrder>(
			&mut self,
			nbytes: usize,
		) -> Result<u64> {
			let mut buf = [0; 8];
			self.read_exact(&mut buf[..nbytes])?;
			Ok(T::read_uint(&buf[..nbytes], nbytes))
		}

		#[inline]
		fn read_int<T: ByteOrder>(
			&mut self,
			nbytes: usize,
		) -> Result<i64> {
			let mut buf = [0; 8];
			self.read_exact(&mut buf[..nbytes])?;
			Ok(T::read_int(&buf[..nbytes], nbytes))
		}

		#[inline]
		fn read_uint128<T: ByteOrder>(
			&mut self,
			nbytes: usize,
		) -> Result<u128> {
			let mut buf = [0; 16];
			self.read_exact(&mut buf[..nbytes])?;
			Ok(T::read_uint128(&buf[..nbytes], nbytes))
		}

		#[inline]
		fn read_int128<T: ByteOrder>(
			&mut self,
			nbytes: usize,
		) -> Result<i128> {
			let mut buf = [0; 16];
			self.read_exact(&mut buf[..nbytes])?;
			Ok(T::read_int128(&buf[..nbytes], nbytes))
		}

		#[inline]
		fn read_f32<T: ByteOrder>(&mut self) -> Result<f32> {
			let mut buf = [0; 4];
			self.read_exact(&mut buf)?;
			Ok(T::read_f32(&buf))
		}

		#[inline]
		fn read_f64<T: ByteOrder>(&mut self) -> Result<f64> {
			let mut buf = [0; 8];
			self.read_exact(&mut buf)?;
			Ok(T::read_f64(&buf))
		}

		#[inline]
		fn read_u16_into<T: ByteOrder>(
			&mut self,
			dst: &mut [u16],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_u16(dst);
			Ok(())
		}

		#[inline]
		fn read_u32_into<T: ByteOrder>(
			&mut self,
			dst: &mut [u32],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_u32(dst);
			Ok(())
		}

		#[inline]
		fn read_u64_into<T: ByteOrder>(
			&mut self,
			dst: &mut [u64],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_u64(dst);
			Ok(())
		}

		#[inline]
		fn read_u128_into<T: ByteOrder>(
			&mut self,
			dst: &mut [u128],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_u128(dst);
			Ok(())
		}

		#[inline]
		fn read_i8_into(
			&mut self,
			dst: &mut [i8],
		) -> Result<()> {
			let buf = unsafe { slice_to_u8_mut(dst) };
			self.read_exact(buf)
		}

		#[inline]
		fn read_i16_into<T: ByteOrder>(
			&mut self,
			dst: &mut [i16],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_i16(dst);
			Ok(())
		}

		#[inline]
		fn read_i32_into<T: ByteOrder>(
			&mut self,
			dst: &mut [i32],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_i32(dst);
			Ok(())
		}

		#[inline]
		fn read_i64_into<T: ByteOrder>(
			&mut self,
			dst: &mut [i64],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_i64(dst);
			Ok(())
		}

		#[inline]
		fn read_i128_into<T: ByteOrder>(
			&mut self,
			dst: &mut [i128],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_i128(dst);
			Ok(())
		}

		#[inline]
		fn read_f32_into<T: ByteOrder>(
			&mut self,
			dst: &mut [f32],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_f32(dst);
			Ok(())
		}

		#[inline]
		#[deprecated(since = "1.2.0", note = "please use `read_f32_into` instead")]
		fn read_f32_into_unchecked<T: ByteOrder>(
			&mut self,
			dst: &mut [f32],
		) -> Result<()> {
			self.read_f32_into::<T>(dst)
		}

		#[inline]
		fn read_f64_into<T: ByteOrder>(
			&mut self,
			dst: &mut [f64],
		) -> Result<()> {
			{
				let buf = unsafe { slice_to_u8_mut(dst) };
				self.read_exact(buf)?;
			}
			T::from_slice_f64(dst);
			Ok(())
		}

		#[inline]
		#[deprecated(since = "1.2.0", note = "please use `read_f64_into` instead")]
		fn read_f64_into_unchecked<T: ByteOrder>(
			&mut self,
			dst: &mut [f64],
		) -> Result<()> {
			self.read_f64_into::<T>(dst)
		}
	}

	impl<R: io::Read + ?Sized> ReadBytesExt for R {}

	pub trait WriteBytesExt: io::Write {
		#[inline]
		fn write_u8(
			&mut self,
			n: u8,
		) -> Result<()> {
			self.write_all(&[n])
		}

		#[inline]
		fn write_i8(
			&mut self,
			n: i8,
		) -> Result<()> {
			self.write_all(&[n as u8])
		}

		#[inline]
		fn write_u16<T: ByteOrder>(
			&mut self,
			n: u16,
		) -> Result<()> {
			let mut buf = [0; 2];
			T::write_u16(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_i16<T: ByteOrder>(
			&mut self,
			n: i16,
		) -> Result<()> {
			let mut buf = [0; 2];
			T::write_i16(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_u24<T: ByteOrder>(
			&mut self,
			n: u32,
		) -> Result<()> {
			let mut buf = [0; 3];
			T::write_u24(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_i24<T: ByteOrder>(
			&mut self,
			n: i32,
		) -> Result<()> {
			let mut buf = [0; 3];
			T::write_i24(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_u32<T: ByteOrder>(
			&mut self,
			n: u32,
		) -> Result<()> {
			let mut buf = [0; 4];
			T::write_u32(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_i32<T: ByteOrder>(
			&mut self,
			n: i32,
		) -> Result<()> {
			let mut buf = [0; 4];
			T::write_i32(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_u48<T: ByteOrder>(
			&mut self,
			n: u64,
		) -> Result<()> {
			let mut buf = [0; 6];
			T::write_u48(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_i48<T: ByteOrder>(
			&mut self,
			n: i64,
		) -> Result<()> {
			let mut buf = [0; 6];
			T::write_i48(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_u64<T: ByteOrder>(
			&mut self,
			n: u64,
		) -> Result<()> {
			let mut buf = [0; 8];
			T::write_u64(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_i64<T: ByteOrder>(
			&mut self,
			n: i64,
		) -> Result<()> {
			let mut buf = [0; 8];
			T::write_i64(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_u128<T: ByteOrder>(
			&mut self,
			n: u128,
		) -> Result<()> {
			let mut buf = [0; 16];
			T::write_u128(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_i128<T: ByteOrder>(
			&mut self,
			n: i128,
		) -> Result<()> {
			let mut buf = [0; 16];
			T::write_i128(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_uint<T: ByteOrder>(
			&mut self,
			n: u64,
			nbytes: usize,
		) -> Result<()> {
			let mut buf = [0; 8];
			T::write_uint(&mut buf, n, nbytes);
			self.write_all(&buf[0..nbytes])
		}

		#[inline]
		fn write_int<T: ByteOrder>(
			&mut self,
			n: i64,
			nbytes: usize,
		) -> Result<()> {
			let mut buf = [0; 8];
			T::write_int(&mut buf, n, nbytes);
			self.write_all(&buf[0..nbytes])
		}

		#[inline]
		fn write_uint128<T: ByteOrder>(
			&mut self,
			n: u128,
			nbytes: usize,
		) -> Result<()> {
			let mut buf = [0; 16];
			T::write_uint128(&mut buf, n, nbytes);
			self.write_all(&buf[0..nbytes])
		}

		#[inline]
		fn write_int128<T: ByteOrder>(
			&mut self,
			n: i128,
			nbytes: usize,
		) -> Result<()> {
			let mut buf = [0; 16];
			T::write_int128(&mut buf, n, nbytes);
			self.write_all(&buf[0..nbytes])
		}

		#[inline]
		fn write_f32<T: ByteOrder>(
			&mut self,
			n: f32,
		) -> Result<()> {
			let mut buf = [0; 4];
			T::write_f32(&mut buf, n);
			self.write_all(&buf)
		}

		#[inline]
		fn write_f64<T: ByteOrder>(
			&mut self,
			n: f64,
		) -> Result<()> {
			let mut buf = [0; 8];
			T::write_f64(&mut buf, n);
			self.write_all(&buf)
		}
	}

	impl<W: io::Write + ?Sized> WriteBytesExt for W {}

	unsafe fn slice_to_u8_mut<T: Copy>(slice: &mut [T]) -> &mut [u8] {
		use std::mem::size_of;

		let len = size_of::<T>() * slice.len();
		unsafe { slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut u8, len) }
	}
}

#[inline]
fn extend_sign(
	val: u64,
	nbytes: usize,
) -> i64 {
	let shift = (8 - nbytes) * 8;
	(val << shift) as i64 >> shift
}

#[inline]
fn extend_sign128(
	val: u128,
	nbytes: usize,
) -> i128 {
	let shift = (16 - nbytes) * 8;
	(val << shift) as i128 >> shift
}

#[inline]
fn unextend_sign(
	val: i64,
	nbytes: usize,
) -> u64 {
	let shift = (8 - nbytes) * 8;
	(val << shift) as u64 >> shift
}

#[inline]
fn unextend_sign128(
	val: i128,
	nbytes: usize,
) -> u128 {
	let shift = (16 - nbytes) * 8;
	(val << shift) as u128 >> shift
}

#[inline]
fn pack_size(n: u64) -> usize {
	if n < 1 << 8 {
		1
	} else if n < 1 << 16 {
		2
	} else if n < 1 << 24 {
		3
	} else if n < 1 << 32 {
		4
	} else if n < 1 << 40 {
		5
	} else if n < 1 << 48 {
		6
	} else if n < 1 << 56 {
		7
	} else {
		8
	}
}

#[inline]
fn pack_size128(n: u128) -> usize {
	if n < 1 << 8 {
		1
	} else if n < 1 << 16 {
		2
	} else if n < 1 << 24 {
		3
	} else if n < 1 << 32 {
		4
	} else if n < 1 << 40 {
		5
	} else if n < 1 << 48 {
		6
	} else if n < 1 << 56 {
		7
	} else if n < 1 << 64 {
		8
	} else if n < 1 << 72 {
		9
	} else if n < 1 << 80 {
		10
	} else if n < 1 << 88 {
		11
	} else if n < 1 << 96 {
		12
	} else if n < 1 << 104 {
		13
	} else if n < 1 << 112 {
		14
	} else if n < 1 << 120 {
		15
	} else {
		16
	}
}

mod private {
	pub trait Sealed {}
	impl Sealed for super::LittleEndian {}
	impl Sealed for super::BigEndian {}
}

pub trait ByteOrder:
	Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + private::Sealed
{
	fn read_u16(buf: &[u8]) -> u16;

	fn read_u24(buf: &[u8]) -> u32 {
		Self::read_uint(buf, 3) as u32
	}

	fn read_u32(buf: &[u8]) -> u32;

	fn read_u48(buf: &[u8]) -> u64 {
		Self::read_uint(buf, 6) as u64
	}

	fn read_u64(buf: &[u8]) -> u64;

	fn read_u128(buf: &[u8]) -> u128;

	fn read_uint(
		buf: &[u8],
		nbytes: usize,
	) -> u64;

	fn read_uint128(
		buf: &[u8],
		nbytes: usize,
	) -> u128;

	fn write_u16(
		buf: &mut [u8],
		n: u16,
	);

	fn write_u24(
		buf: &mut [u8],
		n: u32,
	) {
		Self::write_uint(buf, n as u64, 3)
	}

	fn write_u32(
		buf: &mut [u8],
		n: u32,
	);

	fn write_u48(
		buf: &mut [u8],
		n: u64,
	) {
		Self::write_uint(buf, n as u64, 6)
	}

	fn write_u64(
		buf: &mut [u8],
		n: u64,
	);

	fn write_u128(
		buf: &mut [u8],
		n: u128,
	);

	fn write_uint(
		buf: &mut [u8],
		n: u64,
		nbytes: usize,
	);

	fn write_uint128(
		buf: &mut [u8],
		n: u128,
		nbytes: usize,
	);

	#[inline]
	fn read_i16(buf: &[u8]) -> i16 {
		Self::read_u16(buf) as i16
	}

	#[inline]
	fn read_i24(buf: &[u8]) -> i32 {
		Self::read_int(buf, 3) as i32
	}

	#[inline]
	fn read_i32(buf: &[u8]) -> i32 {
		Self::read_u32(buf) as i32
	}

	#[inline]
	fn read_i48(buf: &[u8]) -> i64 {
		Self::read_int(buf, 6) as i64
	}

	#[inline]
	fn read_i64(buf: &[u8]) -> i64 {
		Self::read_u64(buf) as i64
	}

	#[inline]
	fn read_i128(buf: &[u8]) -> i128 {
		Self::read_u128(buf) as i128
	}

	#[inline]
	fn read_int(
		buf: &[u8],
		nbytes: usize,
	) -> i64 {
		extend_sign(Self::read_uint(buf, nbytes), nbytes)
	}

	#[inline]
	fn read_int128(
		buf: &[u8],
		nbytes: usize,
	) -> i128 {
		extend_sign128(Self::read_uint128(buf, nbytes), nbytes)
	}

	#[inline]
	fn read_f32(buf: &[u8]) -> f32 {
		f32::from_bits(Self::read_u32(buf))
	}

	#[inline]
	fn read_f64(buf: &[u8]) -> f64 {
		f64::from_bits(Self::read_u64(buf))
	}

	#[inline]
	fn write_i16(
		buf: &mut [u8],
		n: i16,
	) {
		Self::write_u16(buf, n as u16)
	}

	#[inline]
	fn write_i24(
		buf: &mut [u8],
		n: i32,
	) {
		Self::write_int(buf, n as i64, 3)
	}

	#[inline]
	fn write_i32(
		buf: &mut [u8],
		n: i32,
	) {
		Self::write_u32(buf, n as u32)
	}

	#[inline]
	fn write_i48(
		buf: &mut [u8],
		n: i64,
	) {
		Self::write_int(buf, n as i64, 6)
	}

	#[inline]
	fn write_i64(
		buf: &mut [u8],
		n: i64,
	) {
		Self::write_u64(buf, n as u64)
	}

	#[inline]
	fn write_i128(
		buf: &mut [u8],
		n: i128,
	) {
		Self::write_u128(buf, n as u128)
	}

	#[inline]
	fn write_int(
		buf: &mut [u8],
		n: i64,
		nbytes: usize,
	) {
		Self::write_uint(buf, unextend_sign(n, nbytes), nbytes)
	}

	#[inline]
	fn write_int128(
		buf: &mut [u8],
		n: i128,
		nbytes: usize,
	) {
		Self::write_uint128(buf, unextend_sign128(n, nbytes), nbytes)
	}

	#[inline]
	fn write_f32(
		buf: &mut [u8],
		n: f32,
	) {
		Self::write_u32(buf, n.to_bits())
	}

	#[inline]
	fn write_f64(
		buf: &mut [u8],
		n: f64,
	) {
		Self::write_u64(buf, n.to_bits())
	}

	fn read_u16_into(
		src: &[u8],
		dst: &mut [u16],
	);

	fn read_u32_into(
		src: &[u8],
		dst: &mut [u32],
	);

	fn read_u64_into(
		src: &[u8],
		dst: &mut [u64],
	);

	fn read_u128_into(
		src: &[u8],
		dst: &mut [u128],
	);

	#[inline]
	fn read_i16_into(
		src: &[u8],
		dst: &mut [i16],
	) {
		let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u16, dst.len()) };
		Self::read_u16_into(src, dst)
	}

	#[inline]
	fn read_i32_into(
		src: &[u8],
		dst: &mut [i32],
	) {
		let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u32, dst.len()) };
		Self::read_u32_into(src, dst);
	}

	#[inline]
	fn read_i64_into(
		src: &[u8],
		dst: &mut [i64],
	) {
		let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u64, dst.len()) };
		Self::read_u64_into(src, dst);
	}

	#[inline]
	fn read_i128_into(
		src: &[u8],
		dst: &mut [i128],
	) {
		let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u128, dst.len()) };
		Self::read_u128_into(src, dst);
	}

	#[inline]
	fn read_f32_into(
		src: &[u8],
		dst: &mut [f32],
	) {
		let dst = unsafe {
			const _: () = assert!(align_of::<u32>() <= align_of::<f32>());
			slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u32, dst.len())
		};
		Self::read_u32_into(src, dst);
	}

	#[inline]
	#[deprecated(since = "1.3.0", note = "please use `read_f32_into` instead")]
	fn read_f32_into_unchecked(
		src: &[u8],
		dst: &mut [f32],
	) {
		Self::read_f32_into(src, dst);
	}

	#[inline]
	fn read_f64_into(
		src: &[u8],
		dst: &mut [f64],
	) {
		let dst = unsafe {
			const _: () = assert!(align_of::<u64>() <= align_of::<f64>());
			slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u64, dst.len())
		};
		Self::read_u64_into(src, dst);
	}

	#[inline]
	#[deprecated(since = "1.3.0", note = "please use `read_f64_into` instead")]
	fn read_f64_into_unchecked(
		src: &[u8],
		dst: &mut [f64],
	) {
		Self::read_f64_into(src, dst);
	}

	fn write_u16_into(
		src: &[u16],
		dst: &mut [u8],
	);

	fn write_u32_into(
		src: &[u32],
		dst: &mut [u8],
	);

	fn write_u64_into(
		src: &[u64],
		dst: &mut [u8],
	);

	fn write_u128_into(
		src: &[u128],
		dst: &mut [u8],
	);

	fn write_i8_into(
		src: &[i8],
		dst: &mut [u8],
	) {
		let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u8, src.len()) };
		dst.copy_from_slice(src);
	}

	fn write_i16_into(
		src: &[i16],
		dst: &mut [u8],
	) {
		let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u16, src.len()) };
		Self::write_u16_into(src, dst);
	}

	fn write_i32_into(
		src: &[i32],
		dst: &mut [u8],
	) {
		let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u32, src.len()) };
		Self::write_u32_into(src, dst);
	}

	fn write_i64_into(
		src: &[i64],
		dst: &mut [u8],
	) {
		let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u64, src.len()) };
		Self::write_u64_into(src, dst);
	}

	fn write_i128_into(
		src: &[i128],
		dst: &mut [u8],
	) {
		let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u128, src.len()) };
		Self::write_u128_into(src, dst);
	}

	fn write_f32_into(
		src: &[f32],
		dst: &mut [u8],
	) {
		let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u32, src.len()) };
		Self::write_u32_into(src, dst);
	}

	fn write_f64_into(
		src: &[f64],
		dst: &mut [u8],
	) {
		let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u64, src.len()) };
		Self::write_u64_into(src, dst);
	}

	fn from_slice_u16(numbers: &mut [u16]);

	fn from_slice_u32(numbers: &mut [u32]);

	fn from_slice_u64(numbers: &mut [u64]);

	fn from_slice_u128(numbers: &mut [u128]);

	#[inline]
	fn from_slice_i16(src: &mut [i16]) {
		let src = unsafe { slice::from_raw_parts_mut(src.as_mut_ptr() as *mut u16, src.len()) };
		Self::from_slice_u16(src);
	}

	#[inline]
	fn from_slice_i32(src: &mut [i32]) {
		let src = unsafe { slice::from_raw_parts_mut(src.as_mut_ptr() as *mut u32, src.len()) };
		Self::from_slice_u32(src);
	}

	#[inline]
	fn from_slice_i64(src: &mut [i64]) {
		let src = unsafe { slice::from_raw_parts_mut(src.as_mut_ptr() as *mut u64, src.len()) };
		Self::from_slice_u64(src);
	}

	#[inline]
	fn from_slice_i128(src: &mut [i128]) {
		let src = unsafe { slice::from_raw_parts_mut(src.as_mut_ptr() as *mut u128, src.len()) };
		Self::from_slice_u128(src);
	}

	fn from_slice_f32(numbers: &mut [f32]);

	fn from_slice_f64(numbers: &mut [f64]);
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BigEndian {}

impl Default for BigEndian {
	fn default() -> BigEndian {
		panic!("BigEndian default")
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LittleEndian {}

impl Default for LittleEndian {
	fn default() -> LittleEndian {
		panic!("LittleEndian default")
	}
}

#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;

#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;

macro_rules! read_slice {
	($src:expr, $dst:expr, $ty:ty, $from_bytes:ident) => {{
		const SIZE: usize = core::mem::size_of::<$ty>();
		let src: &[u8] = $src;
		let dst: &mut [$ty] = $dst;
		assert_eq!(src.len(), dst.len() * SIZE);
		for (src, dst) in src
			.chunks_exact(SIZE)
			.zip(dst.iter_mut())
		{
			*dst = <$ty>::$from_bytes(
				src
					.try_into()
					.unwrap(),
			);
		}
	}};
}

macro_rules! write_slice {
	($src:expr, $dst:expr, $ty:ty, $to_bytes:ident) => {{
		const SIZE: usize = core::mem::size_of::<$ty>();
		let src: &[$ty] = $src;
		let dst: &mut [u8] = $dst;
		assert_eq!(src.len() * SIZE, dst.len());
		for (src, dst) in src
			.iter()
			.zip(dst.chunks_exact_mut(SIZE))
		{
			dst.copy_from_slice(&src.$to_bytes());
		}
	}};
}

impl ByteOrder for BigEndian {
	#[inline]
	fn read_u16(buf: &[u8]) -> u16 {
		u16::from_be_bytes(
			buf[..2]
				.try_into()
				.unwrap(),
		)
	}

	#[inline]
	fn read_u32(buf: &[u8]) -> u32 {
		u32::from_be_bytes(
			buf[..4]
				.try_into()
				.unwrap(),
		)
	}

	#[inline]
	fn read_u64(buf: &[u8]) -> u64 {
		u64::from_be_bytes(
			buf[..8]
				.try_into()
				.unwrap(),
		)
	}

	#[inline]
	fn read_u128(buf: &[u8]) -> u128 {
		u128::from_be_bytes(
			buf[..16]
				.try_into()
				.unwrap(),
		)
	}

	#[inline]
	fn read_uint(
		buf: &[u8],
		nbytes: usize,
	) -> u64 {
		let mut out = [0; 8];
		assert!(1 <= nbytes && nbytes <= out.len() && nbytes <= buf.len());
		let start = out.len() - nbytes;
		out[start..].copy_from_slice(&buf[..nbytes]);
		u64::from_be_bytes(out)
	}

	#[inline]
	fn read_uint128(
		buf: &[u8],
		nbytes: usize,
	) -> u128 {
		let mut out = [0; 16];
		assert!(1 <= nbytes && nbytes <= out.len() && nbytes <= buf.len());
		let start = out.len() - nbytes;
		out[start..].copy_from_slice(&buf[..nbytes]);
		u128::from_be_bytes(out)
	}

	#[inline]
	fn write_u16(
		buf: &mut [u8],
		n: u16,
	) {
		buf[..2].copy_from_slice(&n.to_be_bytes());
	}

	#[inline]
	fn write_u32(
		buf: &mut [u8],
		n: u32,
	) {
		buf[..4].copy_from_slice(&n.to_be_bytes());
	}

	#[inline]
	fn write_u64(
		buf: &mut [u8],
		n: u64,
	) {
		buf[..8].copy_from_slice(&n.to_be_bytes());
	}

	#[inline]
	fn write_u128(
		buf: &mut [u8],
		n: u128,
	) {
		buf[..16].copy_from_slice(&n.to_be_bytes());
	}

	#[inline]
	fn write_uint(
		buf: &mut [u8],
		n: u64,
		nbytes: usize,
	) {
		assert!(pack_size(n) <= nbytes && nbytes <= 8);
		assert!(nbytes <= buf.len());
		unsafe {
			let bytes = *(&n.to_be() as *const u64 as *const [u8; 8]);
			copy_nonoverlapping(
				bytes
					.as_ptr()
					.offset((8 - nbytes) as isize),
				buf.as_mut_ptr(),
				nbytes,
			);
		}
	}

	#[inline]
	fn write_uint128(
		buf: &mut [u8],
		n: u128,
		nbytes: usize,
	) {
		assert!(pack_size128(n) <= nbytes && nbytes <= 16);
		assert!(nbytes <= buf.len());
		unsafe {
			let bytes = *(&n.to_be() as *const u128 as *const [u8; 16]);
			copy_nonoverlapping(
				bytes
					.as_ptr()
					.offset((16 - nbytes) as isize),
				buf.as_mut_ptr(),
				nbytes,
			);
		}
	}

	#[inline]
	fn read_u16_into(
		src: &[u8],
		dst: &mut [u16],
	) {
		read_slice!(src, dst, u16, from_be_bytes);
	}

	#[inline]
	fn read_u32_into(
		src: &[u8],
		dst: &mut [u32],
	) {
		read_slice!(src, dst, u32, from_be_bytes);
	}

	#[inline]
	fn read_u64_into(
		src: &[u8],
		dst: &mut [u64],
	) {
		read_slice!(src, dst, u64, from_be_bytes);
	}

	#[inline]
	fn read_u128_into(
		src: &[u8],
		dst: &mut [u128],
	) {
		read_slice!(src, dst, u128, from_be_bytes);
	}

	#[inline]
	fn write_u16_into(
		src: &[u16],
		dst: &mut [u8],
	) {
		write_slice!(src, dst, u16, to_be_bytes);
	}

	#[inline]
	fn write_u32_into(
		src: &[u32],
		dst: &mut [u8],
	) {
		write_slice!(src, dst, u32, to_be_bytes);
	}

	#[inline]
	fn write_u64_into(
		src: &[u64],
		dst: &mut [u8],
	) {
		write_slice!(src, dst, u64, to_be_bytes);
	}

	#[inline]
	fn write_u128_into(
		src: &[u128],
		dst: &mut [u8],
	) {
		write_slice!(src, dst, u128, to_be_bytes);
	}

	#[inline]
	fn from_slice_u16(numbers: &mut [u16]) {
		if cfg!(target_endian = "little") {
			for n in numbers {
				*n = n.to_be();
			}
		}
	}

	#[inline]
	fn from_slice_u32(numbers: &mut [u32]) {
		if cfg!(target_endian = "little") {
			for n in numbers {
				*n = n.to_be();
			}
		}
	}

	#[inline]
	fn from_slice_u64(numbers: &mut [u64]) {
		if cfg!(target_endian = "little") {
			for n in numbers {
				*n = n.to_be();
			}
		}
	}

	#[inline]
	fn from_slice_u128(numbers: &mut [u128]) {
		if cfg!(target_endian = "little") {
			for n in numbers {
				*n = n.to_be();
			}
		}
	}

	#[inline]
	fn from_slice_f32(numbers: &mut [f32]) {
		if cfg!(target_endian = "little") {
			for n in numbers {
				unsafe {
					let int = *(n as *const f32 as *const u32);
					*n = *(&int.to_be() as *const u32 as *const f32);
				}
			}
		}
	}

	#[inline]
	fn from_slice_f64(numbers: &mut [f64]) {
		if cfg!(target_endian = "little") {
			for n in numbers {
				unsafe {
					let int = *(n as *const f64 as *const u64);
					*n = *(&int.to_be() as *const u64 as *const f64);
				}
			}
		}
	}
}

impl ByteOrder for LittleEndian {
	#[inline]
	fn read_u16(buf: &[u8]) -> u16 {
		u16::from_le_bytes(
			buf[..2]
				.try_into()
				.unwrap(),
		)
	}

	#[inline]
	fn read_u32(buf: &[u8]) -> u32 {
		u32::from_le_bytes(
			buf[..4]
				.try_into()
				.unwrap(),
		)
	}

	#[inline]
	fn read_u64(buf: &[u8]) -> u64 {
		u64::from_le_bytes(
			buf[..8]
				.try_into()
				.unwrap(),
		)
	}

	#[inline]
	fn read_u128(buf: &[u8]) -> u128 {
		u128::from_le_bytes(
			buf[..16]
				.try_into()
				.unwrap(),
		)
	}

	#[inline]
	fn read_uint(
		buf: &[u8],
		nbytes: usize,
	) -> u64 {
		let mut out = [0; 8];
		assert!(1 <= nbytes && nbytes <= out.len() && nbytes <= buf.len());
		out[..nbytes].copy_from_slice(&buf[..nbytes]);
		u64::from_le_bytes(out)
	}

	#[inline]
	fn read_uint128(
		buf: &[u8],
		nbytes: usize,
	) -> u128 {
		let mut out = [0; 16];
		assert!(1 <= nbytes && nbytes <= out.len() && nbytes <= buf.len());
		out[..nbytes].copy_from_slice(&buf[..nbytes]);
		u128::from_le_bytes(out)
	}

	#[inline]
	fn write_u16(
		buf: &mut [u8],
		n: u16,
	) {
		buf[..2].copy_from_slice(&n.to_le_bytes());
	}

	#[inline]
	fn write_u32(
		buf: &mut [u8],
		n: u32,
	) {
		buf[..4].copy_from_slice(&n.to_le_bytes());
	}

	#[inline]
	fn write_u64(
		buf: &mut [u8],
		n: u64,
	) {
		buf[..8].copy_from_slice(&n.to_le_bytes());
	}

	#[inline]
	fn write_u128(
		buf: &mut [u8],
		n: u128,
	) {
		buf[..16].copy_from_slice(&n.to_le_bytes());
	}

	#[inline]
	fn write_uint(
		buf: &mut [u8],
		n: u64,
		nbytes: usize,
	) {
		assert!(pack_size(n as u64) <= nbytes && nbytes <= 8);
		assert!(nbytes <= buf.len());
		unsafe {
			let bytes = *(&n.to_le() as *const u64 as *const [u8; 8]);
			copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr(), nbytes);
		}
	}

	#[inline]
	fn write_uint128(
		buf: &mut [u8],
		n: u128,
		nbytes: usize,
	) {
		assert!(pack_size128(n as u128) <= nbytes && nbytes <= 16);
		assert!(nbytes <= buf.len());
		unsafe {
			let bytes = *(&n.to_le() as *const u128 as *const [u8; 16]);
			copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr(), nbytes);
		}
	}

	#[inline]
	fn read_u16_into(
		src: &[u8],
		dst: &mut [u16],
	) {
		read_slice!(src, dst, u16, from_le_bytes);
	}

	#[inline]
	fn read_u32_into(
		src: &[u8],
		dst: &mut [u32],
	) {
		read_slice!(src, dst, u32, from_le_bytes);
	}

	#[inline]
	fn read_u64_into(
		src: &[u8],
		dst: &mut [u64],
	) {
		read_slice!(src, dst, u64, from_le_bytes);
	}

	#[inline]
	fn read_u128_into(
		src: &[u8],
		dst: &mut [u128],
	) {
		read_slice!(src, dst, u128, from_le_bytes);
	}

	#[inline]
	fn write_u16_into(
		src: &[u16],
		dst: &mut [u8],
	) {
		write_slice!(src, dst, u16, to_le_bytes);
	}

	#[inline]
	fn write_u32_into(
		src: &[u32],
		dst: &mut [u8],
	) {
		write_slice!(src, dst, u32, to_le_bytes);
	}

	#[inline]
	fn write_u64_into(
		src: &[u64],
		dst: &mut [u8],
	) {
		write_slice!(src, dst, u64, to_le_bytes);
	}

	#[inline]
	fn write_u128_into(
		src: &[u128],
		dst: &mut [u8],
	) {
		write_slice!(src, dst, u128, to_le_bytes);
	}

	#[inline]
	fn from_slice_u16(numbers: &mut [u16]) {
		if cfg!(target_endian = "big") {
			for n in numbers {
				*n = n.to_le();
			}
		}
	}

	#[inline]
	fn from_slice_u32(numbers: &mut [u32]) {
		if cfg!(target_endian = "big") {
			for n in numbers {
				*n = n.to_le();
			}
		}
	}

	#[inline]
	fn from_slice_u64(numbers: &mut [u64]) {
		if cfg!(target_endian = "big") {
			for n in numbers {
				*n = n.to_le();
			}
		}
	}

	#[inline]
	fn from_slice_u128(numbers: &mut [u128]) {
		if cfg!(target_endian = "big") {
			for n in numbers {
				*n = n.to_le();
			}
		}
	}

	#[inline]
	fn from_slice_f32(numbers: &mut [f32]) {
		if cfg!(target_endian = "big") {
			for n in numbers {
				unsafe {
					let int = *(n as *const f32 as *const u32);
					*n = *(&int.to_le() as *const u32 as *const f32);
				}
			}
		}
	}

	#[inline]
	fn from_slice_f64(numbers: &mut [f64]) {
		if cfg!(target_endian = "big") {
			for n in numbers {
				unsafe {
					let int = *(n as *const f64 as *const u64);
					*n = *(&int.to_le() as *const u64 as *const f64);
				}
			}
		}
	}
}
