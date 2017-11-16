use std::io;
use core::fmt::Debug;
use core::hash::Hash;
use core::mem::transmute;
use core::ptr::copy_nonoverlapping;

pub trait Sealed {}

impl Sealed for LittleEndian {}

pub trait ByteOrder: Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sealed {
    fn read_u32(buf: &[u8]) -> u32;
    fn read_u64(buf: &[u8]) -> u64;
    fn write_u32(buf: &mut [u8], n: u32);
    fn write_u64(buf: &mut [u8], n: u64);

    #[inline]
    fn read_i32(buf: &[u8]) -> i32 {
        Self::read_u32(buf) as i32
    }

    #[inline]
    fn read_i64(buf: &[u8]) -> i64 {
        Self::read_u64(buf) as i64
    }

    #[inline]
    fn read_f64(buf: &[u8]) -> f64 {
        safe_u64_bits_to_f64(Self::read_u64(buf))
    }

    #[inline]
    fn write_i32(buf: &mut [u8], n: i32) {
        Self::write_u32(buf, n as u32)
    }

    #[inline]
    fn write_i64(buf: &mut [u8], n: i64) {
        Self::write_u64(buf, n as u64)
    }

    #[inline]
    fn write_f64(buf: &mut [u8], n: f64) {
        Self::write_u64(buf, unsafe { transmute(n) })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LittleEndian {}

impl Default for LittleEndian {
    fn default() -> LittleEndian {
        panic!("LittleEndian default")
    }
}

macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => ({
        assert!($size == ::core::mem::size_of::<$ty>());
        assert!($size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            copy_nonoverlapping(
                $src.as_ptr(),
                &mut data as *mut $ty as *mut u8,
                $size);
        }
        data.$which()
    });
}

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $n:expr, $dst:expr, $which:ident) => ({
        assert!($size <= $dst.len());
        unsafe {
            let bytes = transmute::<_, [u8; $size]>($n.$which());
            copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
        }
    });
}

impl ByteOrder for LittleEndian {
    #[inline]
    fn read_u32(buf: &[u8]) -> u32 {
        read_num_bytes!(u32, 4, buf, to_le)
    }

    #[inline]
    fn read_u64(buf: &[u8]) -> u64 {
        read_num_bytes!(u64, 8, buf, to_le)
    }

    #[inline]
    fn write_u32(buf: &mut [u8], n: u32) {
        write_num_bytes!(u32, 4, n, buf, to_le);
    }

    #[inline]
    fn write_u64(buf: &mut [u8], n: u64) {
        write_num_bytes!(u64, 8, n, buf, to_le);
    }
}

#[inline]
fn safe_u64_bits_to_f64(u: u64) -> f64 {
    use core::f64::NAN;

    const EXP_MASK: u64 = 0x7FF0000000000000;
    const FRACT_MASK: u64 = 0x000FFFFFFFFFFFFF;

    if u & EXP_MASK == EXP_MASK && u & FRACT_MASK != 0 {
        NAN
    } else {
        unsafe { transmute(u) }
    }
}

pub trait ReadBytesExt: io::Read {
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_i8(&mut self) -> io::Result<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    #[inline]
    fn read_i32<T: ByteOrder>(&mut self) -> io::Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(T::read_i32(&buf))
    }

    #[inline]
    fn read_i64<T: ByteOrder>(&mut self) -> io::Result<i64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(T::read_i64(&buf))
    }

    #[inline]
    fn read_f64<T: ByteOrder>(&mut self) -> io::Result<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(T::read_f64(&buf))
    }
}

impl<R: io::Read + ?Sized> ReadBytesExt for R {}

pub trait WriteBytesExt: io::Write {
    #[inline]
    fn write_u8(&mut self, n: u8) -> io::Result<()> {
        self.write_all(&[n])
    }

    #[inline]
    fn write_i8(&mut self, n: i8) -> io::Result<()> {
        self.write_all(&[n as u8])
    }

    #[inline]
    fn write_i32<T: ByteOrder>(&mut self, n: i32) -> io::Result<()> {
        let mut buf = [0; 4];
        T::write_i32(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_i64<T: ByteOrder>(&mut self, n: i64) -> io::Result<()> {
        let mut buf = [0; 8];
        T::write_i64(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_f64<T: ByteOrder>(&mut self, n: f64) -> io::Result<()> {
        let mut buf = [0; 8];
        T::write_f64(&mut buf, n);
        self.write_all(&buf)
    }
}

impl<W: io::Write + ?Sized> WriteBytesExt for W {}
