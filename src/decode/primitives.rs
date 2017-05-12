use decode;
use decode::Parse;
use decode::{Result, Error};

use std::fmt;

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
        match str::from_utf8(&self.0[..]) {
            Ok(s) => f.debug_tuple("Tag")
                .field(&s)
                .finish(),
            Err(_) => {
                // Print in Hex if not ascii
                let n = (self.0[3] as u32) << 24
                    | (self.0[2] as u32) << 16
                    | (self.0[1] as u32) << 8
                    | (self.0[0] as u32);

                f.debug_tuple("Tag")
                    .field(&format!("{:X}", n))
                    .finish()
            }
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
pub struct Offset32(u32);

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

#[derive(Debug)]
pub struct Ignore1;
#[derive(Debug)]
pub struct Ignore2;
#[derive(Debug)]
pub struct Ignore4;
#[derive(Debug)]
pub struct Ignore6;
#[derive(Debug)]
pub struct Ignore8;
#[derive(Debug)]
pub struct Ignore16;

macro_rules! impl_ignore {
    ($($size:expr => $name:ident),*) => (
        $(
            impl Parse for $name {
                fn static_size() -> usize { $size }
                fn parse(buf: &[u8]) -> Result<(&[u8], Self)> {
                    if buf.len() < Self::static_size() {
                        return Err(Error::UnexpectedEof)
                    }

                    Ok(( &buf[$size..], $name ))
                }
            }
        )*
    )
}

impl_ignore!(
    1 => Ignore1,
    2 => Ignore2,
    4 => Ignore4,
    6 => Ignore6,
    8 => Ignore8,
    16 => Ignore16
);