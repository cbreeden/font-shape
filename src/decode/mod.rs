use std::result;
pub mod primitives;

#[derive(Debug)]
pub enum Error {
    InvalidData,
    UnexpectedEof,
    TtcfUnsupported,
}

pub type Result<T> = result::Result<T, Error>;

/// Trait used to deserialize primitive types.  This method may panic.
pub trait Primitive: Sized {
    fn size() -> usize;
    /// This method panics if there are not enough bytes.
    fn parse(&[u8]) -> Result<Self>;
}

pub trait ReadPrimitive {
    fn read<T: Primitive>(&mut self) -> Result<T>;
}

impl<'a> ReadPrimitive for &'a [u8] {
    fn read<T: Primitive>(&mut self) -> Result<T> {
        if self.len() < T::size() {
            return Err(Error::UnexpectedEof)
        }

        let result = T::parse(self);
        *self = &self[T::size()..];
        result
    }
}

pub trait SizedTable {
    fn size() -> usize;
}

pub trait Table<'tbl>: Sized {
    fn parse(&'tbl [u8]) -> Result<Self>;
}

pub trait ReadTable<'tbl> {
    fn read_table<T: Table<'tbl> + SizedTable>(&mut self) -> Result<T>;
}

impl<'a: 'tbl, 'tbl> ReadTable<'tbl> for &'a [u8] {
    fn read_table<T: Table<'tbl> + SizedTable>(&mut self) -> Result<T> {
        if self.len() < T::size() {
            return  Err(Error::UnexpectedEof)
        }

        let result = T::parse(self);
        *self = &self[T::size()..];
        result
    }
}