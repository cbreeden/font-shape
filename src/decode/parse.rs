use std::result;

#[derive(Debug)]
pub enum Error {
    InvalidData,
    UnexpectedEof,
    TtcfUnsupported,
}

pub type Result<T> = result::Result<T, Error>;

pub trait Parse
    where Self: Sized {
    fn static_size() -> usize;
    fn parse(&[u8]) -> Result<(&[u8], Self)>;
}

/// Panics if buf.len() == 0
pub fn be_u8(buf: &[u8]) -> u8 {
    buf[0]
}

/// Panics if buf.len() == 0
pub fn be_i8(buf: &[u8]) -> i8 {
    buf[0] as i8
}

/// Panics if buf.len() < 2.
pub fn be_u16(buf: &[u8]) -> u16 {
    ((buf[0] as u16) << 8)
    | (buf[1] as u16)
}

/// Panics if buf.len() < 2.
pub fn be_i16(buf: &[u8]) -> i16 {
    be_u16(buf) as i16
}

/// Panics if buf.len() < 4.
pub fn be_u32(buf: &[u8]) -> u32 {
    ((buf[0] as u32) << 24)
    | ((buf[1] as u32) << 16)
    | ((buf[2] as u32) << 8)
    |  (buf[3] as u32)
}

/// Panics if buf.len() < 4.
pub fn be_i32(buf: &[u8]) -> i32 {
    be_u32(buf) as i32
}

/// Panics if buf.len() < 4.
pub fn be_u8_4(buf: &[u8]) -> [u8; 4] {
    [buf[0], buf[1], buf[2], buf[3]]
}