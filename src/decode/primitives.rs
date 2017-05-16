use decode;
use decode::Table;
use decode::StaticSize;
use decode::{Result, Error};

use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fixed(pub f64);

impl From<i32> for Fixed {
    fn from(n: i32) -> Fixed {
        let fp = (n as f64) / (2u16.pow(16) as f64);
        Fixed(fp)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F2Dot14(pub f32);

impl From<i16> for F2Dot14 {
    fn from(n: i16) -> F2Dot14 {
        let fp = (n as f32) / (2u16.pow(14) as f32);
        F2Dot14(fp)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FWord(pub i16);

impl From<i16> for FWord {
    fn from(n: i16) -> FWord {
        FWord(n)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UFWord(pub u16);

impl From<u16> for UFWord {
    fn from(n: u16) -> UFWord {
        UFWord(n)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tag(pub [u8; 4]);

impl From<[u8; 4]> for Tag {
    fn from(b: [u8; 4]) -> Tag {
        Tag(b)
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
            let n = (self.0[3] as u32) << 24
                | (self.0[2] as u32) << 16
                | (self.0[1] as u32) << 8
                | (self.0[0] as u32);

            f.debug_tuple("Tag")
                .field(&format!("0x{:X}", n))
                .finish()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset16(u16);

impl From<u16> for Offset16 {
    fn from(n: u16) -> Offset16 {
        Offset16(n)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset32(pub u32);

impl From<u32> for Offset32 {
    fn from(n: u32) -> Offset32 {
        Offset32(n)
    }
}

impl_parse!(be_u8_4 => Tag; 4);

impl_parse!(
    be_i32 => Fixed;    4,
    be_i16 => F2Dot14;  2,
    be_i16 => FWord;    2,
    be_u16 => UFWord;   2,
    be_u8  => u8;       1,
    be_i8  => i8;       1,
    be_u16 => u16;      2,
    be_i16 => i16;      2,
    be_u32 => u32;      4,
    be_i32 => i32;      4,
    be_u16 => Offset16; 2,
    be_u32 => Offset32; 4
);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reserved<T> {
    _phantom: PhantomData<T>,
}

impl<T> StaticSize for Reserved<T> where T: StaticSize {
    fn static_size() -> usize { T::static_size() }
}

impl<T> Table for Reserved<T> where T: StaticSize {
    fn parse(buf: &[u8]) -> Result<(&[u8], Self)> {
        Ok((buf, Reserved { _phantom: PhantomData }))
    }
}