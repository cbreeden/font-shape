use decode;
use decode::primitives::{Tag, Offset32, Ignore6};
use decode::Parse;
use decode::{Error, Result};

#[derive(Debug)]
pub enum Version {
    OpenType,
    TrueType,
}

impl Parse for Version {
    fn size() -> usize { 4 }
    fn parse(buf: &[u8]) -> Result<(&[u8], Version)> {
        const VERSION1: [u8; 4] = [0x00, 0x01, 0x00, 0x00];

        if buf.len() < Self::size() {
            return Err(Error::UnexpectedEof)
        }

        let (buf, tag) = Tag::parse(buf)?;
        let ver = match &tag.0 {
            b"OTTO" => Version::OpenType,
            &VERSION1 | b"true" | b"typ1" => Version::TrueType,
            _ => return Err(Error::InvalidData),
        };

        Ok((buf, ver))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableTag {
    // Required Tables
    Cmap,
    Head,
    HorizontalHeader,
    HorizontalMetrics,
    MaximumProfile,
    Names,
    WindowsSpecific,
    Postscript,
    Glyphs,
    Cff,
    Cff2,
    Unsupported(u32),
}

impl From<u32> for TableTag {
    fn from(n: u32) -> TableTag {
        const CMAP: u32 = 0x636d6170;
        const HEAD: u32 = 0x68656164;
        const HHEA: u32 = 0x68686561;
        const HMTX: u32 = 0x686d7478;
        const MAXP: u32 = 0x6d617870;
        const NAME: u32 = 0x6e616d65;
        const OS2:  u32 = 0x4f532f32;
        const POST: u32 = 0x706f7374;

        // TrueType Related
        const GLYF: u32 = 0x676C7966;

        // PostScript Related
        const CFF:  u32 = 0x43464620; // "CFF "
        const CFF2: u32 = 0x43464632; // "CFF2"

        match n {
            CMAP => TableTag::Cmap,
            HEAD => TableTag::Head,
            HHEA => TableTag::HorizontalHeader,
            HMTX => TableTag::HorizontalMetrics,
            MAXP => TableTag::MaximumProfile,
            NAME => TableTag::Names,
            OS2  => TableTag::WindowsSpecific,
            POST => TableTag::Postscript,
            GLYF => TableTag::Glyphs,
            CFF  => TableTag::Cff,
            CFF2 => TableTag::Cff2,
            _    => TableTag::Unsupported(n),
        }
    }
}

impl_parse!(be_u32 => TableTag; 4);

#[derive(Debug)]
struct OffsetTable {
    tag:       TableTag,
    check_sum: u32,
    offset:    Offset32,
    length:    u32,
}

impl Parse for OffsetTable {
    fn size() -> usize {
        TableTag::size() + u32::size() + Offset32::size() + u32::size()
     }
    fn parse(buf: &[u8]) -> Result<(&[u8], Self)> {
        if buf.len() < Self::size() {
            return Err(Error::UnexpectedEof)
        }

        let (buf, tag) = TableTag::parse(buf)?;
        let (buf, check_sum) = u32::parse(buf)?;
        let (buf, offset) = Offset32::parse(buf)?;
        let (buf, length) = u32::parse(buf)?;

        Ok((buf, OffsetTable {
            tag: tag,
            check_sum: check_sum,
            offset: offset,
            length: length,
        }))
    }
}

#[derive(Debug)]
pub struct Font {
    pub version: Version,
    num_tables:  u16,

    _other: Ignore6,
    // search_range:   u16,
    // entry_selector: u16,
    // range_shift:    u16
}

impl Parse for Font {
    #[inline]
    fn size() -> usize {
        12
    }

    fn parse(buf: &[u8]) -> Result<(&[u8], Self)> {
        if buf.len() < Self::size() {
            return Err(Error::UnexpectedEof)
        }

        let (buf, version) = Version::parse(buf)?;
        let (buf, num_tables) = u16::parse(buf)?;
        let (buf, _) = Ignore6::parse(buf)?;

        Ok((buf, Font {
            version: version,
            num_tables: num_tables,
            _other: Ignore6,
        }))
    }
}

impl Font {
    fn table_iter<'a> (&self, buf: &'a [u8]) -> TableIter<'a> {
            TableIter {
                buf: buf,
                pos: 0,
                max: self.num_tables as usize,
            }
    }
}

struct TableIter<'a> {
    buf: &'a [u8],
    pos: usize,
    max: usize,
}

impl<'a> Iterator for TableIter<'a> {
    type Item = Result<OffsetTable>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.max {
            return None
        }

        let next = match OffsetTable::parse(self.buf) {
            Err(e) => return Some(Err(e)),
            Ok(n)  => n,
        };

        self.buf = next.0;
        self.pos += 1;

        Some(Ok(next.1))
    }
}

#[cfg(test)]
mod test {
    use super::Font;
    use super::Parse;
    use super::OffsetTable;

    #[test]
    fn print_tables() {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let file = File::open(r"data/OpenSans-Regular.ttf")
            .expect("Unable to open file");

        let mut reader = BufReader::new(file);
        let mut data   = Vec::new();
        reader.read_to_end(&mut data)
            .expect("Error reading file");

        let (data, font) = Font::parse(&data)
            .expect("Unable to parse font");

        for table in font.table_iter(data) {
            println!("{:?}", table.unwrap());
        }

        println!("{:?}", font);
    }
}