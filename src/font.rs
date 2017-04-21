use parse;
use parse::Parse;
use parse::{Error, Result};

#[derive(Debug)]
pub enum Version {
    OpenType,
    TrueType,
}

impl Parse for Version {
    fn size() -> usize { 4 }
    fn parse(buf: &[u8]) -> Result<(&[u8], Version)> {
        const TRUE: u32     = 0x74727565; // deprecated Mac format (TrueType)
        const TYP1: u32     = 0x74797031; // deprecated Mac format (TrueType)
        const VERSION1: u32 = 0x00010000; // TrueType
        const OTTO: u32     = 0x4f54544f; // OpenType

        if buf.len() < Self::size() {
            return Err(Error::UnexpectedEof)
        }

        let val = parse::be_u32(buf);
        let buf = &buf[Self::size()..];
        Ok((buf, match val {
            OTTO => Version::OpenType,
            VERSION1 | TRUE | TYP1 => Version::TrueType,

            // TODO: Investigate if providing a default is fine.
            _ => return Err(Error::InvalidData),
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
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

impl From<u32> for Tag {
    fn from(n: u32) -> Tag {
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
            CMAP => Tag::Cmap,
            HEAD => Tag::Head,
            HHEA => Tag::HorizontalHeader,
            HMTX => Tag::HorizontalMetrics,
            MAXP => Tag::MaximumProfile,
            NAME => Tag::Names,
            OS2  => Tag::WindowsSpecific,
            POST => Tag::Postscript,
            GLYF => Tag::Glyphs,
            CFF  => Tag::Cff,
            CFF2 => Tag::Cff2,
            _    => Tag::Unsupported(n),
        }
    }
}

impl Parse for Tag {
    fn size() -> usize { 4 }

    fn parse(buf: &[u8]) -> Result<(&[u8], Self)> {
        if buf.len() < Self::size() {
            return Err(Error::UnexpectedEof)
        }

        let tag = parse::be_u32(buf).into();
        let buf = &buf[Self::size()..];

        Ok((buf, tag))
    }
}

#[derive(Debug)]
struct OffsetTable {
    tag:       Tag,
    check_sum: u32,
    //offset:    Offset<'a>,
    length:    u32,
}

impl Parse for OffsetTable {
    fn size() -> usize { 16 }
    fn parse(buf: &[u8]) -> Result<(&[u8], Self)> {
        if buf.len() < Self::size() {
            return Err(Error::UnexpectedEof)
        }

        let (buf, tag) = Tag::parse(buf)?;
        let (buf, check_sum) = u32::parse(buf)?;

        let buf = &buf[4..]; // skip offset for now

        let (buf, length) = u32::parse(buf)?;

        Ok((buf, OffsetTable {
            tag: tag,
            check_sum: check_sum,
            length: length,
        }))
    }
}

#[derive(Debug)]
pub struct Font {
    pub version: Version,
    num_tables:  u16,
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

        let buf = &buf[6..]; // Ignore rest

        Ok((buf, Font {
            version: version,
            num_tables: num_tables,
        }))
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

        let file = File::open(r"C:\Users\breeden\git\fonttools\data\Kleymissky_0283.otf")
            .expect("Unable to open file");
        let mut reader = BufReader::new(file);
        let mut data   = Vec::new();
        reader.read_to_end(&mut data)
            .expect("Error reading file");

        let (mut data, font) = Font::parse(&data)
            .expect("Unable to parse font");

        for _ in 0..font.num_tables {
            let res   = OffsetTable::parse(data)?;
            data = res.0;

            println!("{:?}", res.0);
        }

        println!("{:?}", font);
    }
}