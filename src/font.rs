use decode::{Error, Result, SizedTable, Table, Primitive, ReadPrimitive, ReadTable};
use decode::primitives::{Tag, Ignored};

use table::TaggedTable;
use table::name::Name;

#[derive(Debug)]
pub enum Version {
    OpenType,
    TrueType,
}

impl Primitive for Version {
    fn size() -> usize {
        Tag::size()
    }
    fn parse(buffer: &[u8]) -> Result<Version> {
        const VERSION1: [u8; 4] = [0x00, 0x01, 0x00, 0x00];
        let tag = Tag::parse(buffer)?;
        match &tag.0 {
            b"OTTO" => Ok(Version::OpenType),

            &VERSION1 | b"true" | b"typ1" => Ok(Version::TrueType),

            b"ttcf" => Err(Error::TtcfUnsupported),
            _ => Err(Error::InvalidData),
        }
    }
}

#[derive(Table, Debug)]
struct OffsetTable {
    sfnt_version: Version,
    num_tables: u16,
    search_range: Ignored<u16>,
    entry_selector: Ignored<u16>,
    range_shift: Ignored<u16>,
}

#[derive(Table, Debug, PartialEq)]
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
        if buf.len() < OffsetTable::size() {
            return Err(Error::InvalidData);
        }

        let offset_table = OffsetTable::parse(buf)?;

        Ok(Font {
               buf: buf,
               num_tables: offset_table.num_tables,
               version: offset_table.sfnt_version,
           })
    }

    pub fn tables(&self) -> Result<TableIter> {
        let shift = OffsetTable::size();
        let required_size = shift + TableRecord::size() * self.num_tables as usize;

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

    pub fn get_table<'tbl, T: TaggedTable<'tbl>>(&'tbl self) -> Option<T> {
        let offset = match self.get_table_offset(T::tag()) {
            Some(offset) => offset,
            None => return None,
        };

        let buf = &self.buf[offset..];

        match T::parse(buf) {
            Ok(tbl) => Some(tbl),
            Err(_) => None,
        }
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
        self.pos += 1;
        let next = self.buf.read_table::<TableRecord>().unwrap();
        Some(next)
    }
}

#[cfg(test)]
mod test {
    use super::Font;
    use decode::primitives::Tag;

    macro_rules! assert_records_eq {
        ( $tbls:ident, $($tag:expr, check_sum: $check_sum:expr, offset: $offset:expr, length: $length:expr),* $(,)* ) => {
            $(
            match $tbls.next() {
                Some(tbl) => {
                    assert_eq!(tbl.tag, Tag($tag));
                    assert_eq!(tbl.check_sum, $check_sum);
                    assert_eq!(tbl.offset, $offset);
                    assert_eq!(tbl.length, $length);
                },

                _ => {
                    panic!("Fewer tables than expected!")
                }
            }
            )*

            assert_eq!($tbls.next(), None, "More tables than expected!");
        }
    }

    #[test]
    fn font_table_records() {
        let buf = open_font!(r"data/OpenSans-Regular.ttf");
        let font = Font::from_buffer(&buf).expect("Unable to parse font");

        let mut tbls = font.tables().expect("Unable to read tables");

        assert_records_eq!(tbls,
            *b"DSIG", check_sum: 2651997213, offset: 211868, length: 5492,
            *b"GDEF", check_sum: 2491311, offset: 210812, length: 30,
            *b"GPOS", check_sum: 188157751, offset: 210844, length: 56,
            *b"GSUB", check_sum: 237714871, offset: 210900, length: 966,
            *b"OS/2", check_sum: 2705235657, offset: 440, length: 96,
            *b"cmap", check_sum: 699084648, offset: 4276, length: 1050,
            *b"cvt ", check_sum: 256710820, offset: 7568, length: 162,
            *b"fpgm", check_sum: 2120332817, offset: 5328, length: 1972,
            *b"gasp", check_sum: 1376291, offset: 210796, length: 16,
            *b"glyf", check_sum: 1949866315, offset: 9612, length: 77748,
            *b"head", check_sum: 4151763622, offset: 316, length: 54,
            *b"hhea", check_sum: 231475571, offset: 372, length: 36,
            *b"hmtx", check_sum: 3895803101, offset: 536, length: 3738,
            *b"kern", check_sum: 1412106622, offset: 87360, length: 112182,
            *b"loca", check_sum: 689233137, offset: 7732, length: 1878,
            *b"maxp", check_sum: 88277514, offset: 408, length: 32,
            *b"name", check_sum: 1940949125, offset: 199544, length: 1479,
            *b"post", check_sum: 38006636, offset: 201024, length: 9771,
            *b"prep", check_sum: 1136105124, offset: 7300, length: 265,        
        );
    }
}