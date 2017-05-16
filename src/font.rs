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
versioned_table!(Version,
    Tag => |tag| {
        const VERSION1: [u8; 4] = [0x00, 0x01, 0x00, 0x00];
        match &tag.0 {
            b"OTTO" => Version::OpenType,
            &VERSION1 | b"true" | b"typ1" => Version::TrueType,
            b"ttcf" => return Err(Error::TtcfUnsupported),
            _ => return Err(Error::InvalidData),
        }
    }
);

#[derive(Table, Debug)]
struct OffsetTable {
    sfnt_version: Version,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

#[derive(Table, Debug)]
struct TableRecord {
    tag:       Tag,
    check_sum: u32,
    offset:    Offset32,
    length:    u32,
}

#[derive(Debug)]
struct Font<'a> {
    buf: &'a [u8],
    version: Version,
    num_tables: u16,
}

impl<'f> Font<'f> {
    fn from_buffer<'b: 'f>(buf: &'b [u8]) -> Result<Font<'f>> {
        if buf.len() < OffsetTable::static_size() {
            return Err(Error::InvalidData)
        }

        let (_, offset_table) = OffsetTable::parse(buf)?;

        Ok(Font {
            buf: buf,
            num_tables: offset_table.num_tables,
            version: offset_table.sfnt_version,
        })
    }

    fn tables(&self) -> TableIter {
        let shift = OffsetTable::static_size();
        TableIter {
            buf: &self.buf[shift..],
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
    type Item = Result<TableRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.max {
            return None
        }

        let next = match TableRecord::parse(self.buf) {
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

        let font = Font::from_buffer(&data)
            .expect("Unable to parse font");

        for tbl in font.tables() {
            let tbl = tbl.unwrap();
            println!("{:?}", tbl);
        }
    }
}