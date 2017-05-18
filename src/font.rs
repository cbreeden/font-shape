use decode::primitives::Tag;
use decode::Table;
use decode::StaticSize;
use decode::{Error, Result};

use table::name::Name;

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
pub struct TableRecord {
    pub tag: Tag,
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug)]
pub struct Font<'a> {
    buf: &'a [u8],
    version: Version,
    num_tables: u16,
}

impl<'f> Font<'f> {
    pub fn from_buffer<'b: 'f>(buf: &'b [u8]) -> Result<Font<'f>> {
        if buf.len() < OffsetTable::static_size() {
            return Err(Error::InvalidData);
        }

        let (_, offset_table) = OffsetTable::parse(buf)?;

        Ok(Font {
               buf: buf,
               num_tables: offset_table.num_tables,
               version: offset_table.sfnt_version,
           })
    }

    pub fn tables(&self) -> Result<TableIter> {
        let shift = OffsetTable::static_size();
        let required_size = shift + TableRecord::static_size() * self.num_tables as usize;

        if self.buf.len() < required_size {
            return Err(Error::InvalidData);
        }

        Ok(TableIter {
               buf: &self.buf[shift..],
               pos: 0,
               max: self.num_tables as usize,
           })
    }

    pub fn get_table_record(&self, tag: Tag) -> Option<TableRecord> {
        let mut tables = match self.tables() {
            Err(_) => return None,
            Ok(tables) => tables,
        };

        tables.find(|tbl| tbl.tag == tag)
    }

    pub fn get_table_offset(&self, tag: Tag) -> Option<usize> {
        match self.get_table_record(tag) {
            None => None,
            Some(table) => Some(table.offset as usize),
        }
    }

    pub fn get_name_table(&self) -> Result<Name> {
        // Get the name table.
        let offset = self.get_table_offset(Tag(*b"name"))
            .ok_or(Error::InvalidData)?;

        let name_buf = &self.buf[offset as usize..];
        let (_, tbl) = Name::parse(name_buf)?;
        Ok(tbl)
    }
}

pub struct TableIter<'a> {
    buf: &'a [u8],
    pos: usize,
    max: usize,
}

impl<'a> Iterator for TableIter<'a> {
    type Item = TableRecord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.max {
            return None;
        }

        // The only possible failure is EOF, which is checked
        // for while constructing TableIter.
        let next = TableRecord::parse(self.buf).unwrap();
        self.buf = next.0;
        self.pos += 1;

        Some(next.1)
    }
}

// #[cfg(test)]
// mod test {
//     use super::Font;
//     use ::decode::primitives::Tag;

//     #[test]
//     fn print_tables() {
//         use std::fs::File;
//         use std::io::BufReader;
//         use std::io::prelude::*;

//         let file = File::open(r"data/OpenSans-Regular.ttf")
//             .expect("Unable to open file");

//         let mut reader = BufReader::new(file);
//         let mut data   = Vec::new();
//         reader.read_to_end(&mut data)
//             .expect("Error reading file");

//         let font = Font::from_buffer(&data)
//             .expect("Unable to parse font");

//         for tbl in font.tables() {
//             println!("{:?}", tbl.unwrap());
//         }
//     }

//     #[test]
//     fn test_tag() {
//         let t = Tag([0x00,0x01,0x00,0x00]);
//         println!("{:?}", t);
//     }
// }