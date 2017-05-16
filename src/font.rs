use decode::primitives::{Tag, Offset32, Reserved};
use decode::Table;
use decode::StaticSize;
use decode::{Error, Result};

use hhea;

#[derive(Debug)]
pub enum Version {
    OpenType,
    TrueType,
}

static_size!(Version = 4);

impl Table for Version {
    fn parse(buf: &[u8]) -> Result<(&[u8], Version)> {
        const VERSION1: [u8; 4] = [0x00, 0x01, 0x00, 0x00];

        if buf.len() < Self::static_size() {
            return Err(Error::UnexpectedEof)
        }

        let (buf, tag) = Tag::parse(buf)?;
        let ver = match &tag.0 {
            b"OTTO" => Version::OpenType,
            &VERSION1 | b"true" | b"typ1" => Version::TrueType,
            b"ttcf" => return Err(Error::TtcfUnsupported),
            _ => return Err(Error::InvalidData),
        };

        Ok((buf, ver))
    }
}

#[derive(Debug, Table)]
struct OffsetTable {
    tag:       Tag,
    check_sum: u32,
    offset:    Offset32,
    length:    u32,
}

#[derive(Debug, Table)]
struct Font<'a> {
    buf: &'a [u8],
    version: Version,
    num_tables: u16,
    search_range: Reserved<u16>,
    entry_selector: Reserved<u16>,
    range_shirt: Reserved<u16>,
    // search_range:   u16,
    // entry_selector: u16,
    // range_shift:    u16
}

impl<'a> Font<'a> {
    fn table_iter<'b> (&'b self) -> TableIter<'b> {
            TableIter {
                buf: self.buf,
                pos: 0,
                max: self.num_tables as usize,
            }
    }

    fn get_table_offset(&self, tag: Tag, buf: &[u8]) -> Option<u32> {
        for table in self.table_iter() {
            let tbl = table.unwrap();
            if tbl.tag == tag {
                return Some(tbl.offset.0)
            }
        }
        None
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
    use super::Table;
    use super::hhea;
    use ::decode::primitives::Tag;

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

        let (offset, font) = Font::parse(&data)
            .expect("Unable to parse font");

        for table in font.table_iter() {
            println!("{:?}", table.unwrap());
        }

        let offset = font.get_table_offset(Tag(*b"hhea"), &offset)
            .expect("unable to find hhea table!");

        let hhea_buf = &data[offset as usize..];
        let (_, hhea) = hhea::Hhea::parse(hhea_buf).unwrap();
        println!("{:?}", hhea);

        println!("{:?}", font);
    }
}