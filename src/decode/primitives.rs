use byteorder::{BigEndian, ByteOrder};
use decode::{Error, Result, Primitive};
use std::fmt;
use std::marker::PhantomData;

macro_rules! impl_data_type {
    ($($conv:expr => $data_type:tt),* $(,)*) => (
        $(
            impl Primitive for $data_type {
                fn size() -> usize { ::std::mem::size_of::<$data_type>() }
                fn parse(buffer: &[u8]) -> Result<$data_type> {
                    Ok($data_type::from($conv(buffer)))
                }
            }
        )*
    );
}

fn read_u8(buffer: &[u8]) -> u8 {
     buffer[0]
}

fn read_i8(buffer: &[u8]) -> i8 {
     buffer[0] as i8
}


impl_data_type!(
    BigEndian::read_i16 => FWord,
    BigEndian::read_u16 => UFWord,
    BigEndian::read_i16 => F2Dot14,
    BigEndian::read_i32 => Fixed,
    BigEndian::read_u16 => Offset16,
    BigEndian::read_u32 => Offset32,

    read_u8  => u8,
    read_i8  => i8,
    BigEndian::read_u16 => u16,
    BigEndian::read_i16 => i16,
    BigEndian::read_u32 => u32,
    BigEndian::read_i32 => i32,
);

// A 32-bit signed fixed-point number (16.16)
// pub struct Fixed(i32);
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, From)]
pub struct Fixed(pub i32);

/// A signed unit to describe a quantity in a font's internal design units.
/// The scale is described in ___.  This is often 1/1000 EMs.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, From)]
pub struct FWord(pub i16);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, From)]
pub struct F2Dot14(pub i16);

/// An usigned unit to describe a quantity in a font's internal design units.
/// The scale is described in ___.  This is often 1/1000 EMs.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, From)]
pub struct UFWord(pub u16);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, From)]
pub struct Offset16(pub u16);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, From)]
pub struct Offset32(pub u32);

/// Usually used to identify a table name, script, language system, or feature.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag(pub [u8; 4]);

impl From<u32> for Tag {
    fn from(n: u32) -> Tag {
        let mut tag = [0; 4];
        BigEndian::write_u32(&mut tag, n);
        Tag(tag)
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ::std::str;
        // Print the ASCII name if the name contains only
        // visible ASCII characters.  Otherwise Hex.
        if self.0.iter().all(|&c| c >= 32 && c <= 128) {
            let s = str::from_utf8(&self.0[..]).unwrap();
            f.debug_tuple("Tag")
                .field(&s)
                .finish()
        } else {
            let n = (self.0[0] as u32) << 24
                | (self.0[1] as u32) << 16
                | (self.0[2] as u32) << 8
                | (self.0[3] as u32);

            write!(f, "Tag(0x{:08X})", n)
        }
    }
}

impl Primitive for Tag {
    fn size() -> usize { 4 }
    fn parse(buffer: &[u8]) -> Result<Self> {
        let mut tag = [0; 4];
        tag.copy_from_slice(&buffer[..4]);
        Ok(Tag(tag))
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct U24(pub u32);

impl Primitive for U24 {
    fn size() -> usize { 3 }
    fn parse(buffer: &[u8]) -> Result<Self> {
        Ok(U24(BigEndian::read_uint(buffer, 3) as u32))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ignored<T> {
    _phantom: PhantomData<T>,
}

impl<T> Primitive for Ignored<T> where T: Primitive {
    fn size() -> usize { T::size() }
    fn parse(_: &[u8]) -> Result<Self> {
        Ok(Ignored { _phantom: PhantomData })
    }
}

impl<T> fmt::Debug for Ignored<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ignored")
    }
}