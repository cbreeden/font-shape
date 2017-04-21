use std::result;

#[derive(Debug)]
pub enum Error {
    InvalidData,
    UnexpectedEof,
}

pub type Result<T> = result::Result<T, Error>;

pub trait Parse
    where Self: Sized {
    fn size() -> usize;
    fn parse(&[u8]) -> Result<(&[u8], Self)>;
}

/// Panics if buf.len() < 2.
pub fn be_u16(buf: &[u8]) -> u16 {
    ((buf[0] as u16) << 8)
    | (buf[1] as u16)
}

/// Panics if buf.len() < 2.
pub fn be_u32(buf: &[u8]) -> u32 {
    ((buf[0] as u32) << 24)
    | ((buf[1] as u32) << 16)
    | ((buf[2] as u32) << 8)
    |  (buf[3] as u32)
}

pub fn be_i16(buf: &[u8]) -> i16 {
    be_u16(buf) as i16
}

pub fn be_i32(buf: &[u8]) -> i32 {
    be_u32(buf) as i32
}

impl Parse for u16 {
    fn size() -> usize { 2 }

    fn parse(buf: &[u8]) -> Result<(&[u8], Self)> {
        if buf.len() < Self::size() {
            return Err(Error::UnexpectedEof)
        }

        let n = be_u16(buf);
        let buf = &buf[Self::size()..];

        Ok((buf, n))
    }
}

impl Parse for u32 {
    fn size() -> usize { 4 }

    fn parse(buf: &[u8]) -> Result<(&[u8], Self)> {
        if buf.len() < Self::size() {
            return Err(Error::UnexpectedEof)
        }

        let n = be_u32(buf);
        let buf = &buf[Self::size()..];

        Ok((buf, n))
    }
}