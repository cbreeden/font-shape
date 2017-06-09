use decode::{Error, Result, SizedTable, Table, TableInherited, Primitive, ReadPrimitive, ReadTable};
use decode::primitives::Ignored;

pub enum Cmap<'a> {
    Format0(Format0<'a>), //How to handle encoding?
    Format4(Format4<'a>),
    Format6(Format6<'a>),
    Format12(Format12<'a>),
    //Format14(Format14), Require seperate api?
}

impl<'a> Cmap<'a> {
    fn format(&self) -> usize {
        match *self {
            Cmap::Format0(_) => 0,
            Cmap::Format4(_) => 4,
            Cmap::Format6(_) => 6,
            Cmap::Format12(_) => 12,
        }
    }

    fn get_glyph_id(&self, codepoint: u32) -> Option<u16> {
        match *self {
            Cmap::Format4(ref cm) => cm.get_glyph_id(codepoint),
            Cmap::Format6(ref cm) => cm.get_glyph_id(codepoint),
            Cmap::Format12(ref cm) => cm.get_glyph_id(codepoint),
            _ => panic!("this cmap format is not implemented"),
        }
    }
}

// impl<'tbl> Table<'tbl> for Cmap<'tbl> {
//     fn parse(buffer: &[u8]) -> Result<Cmap<'tbl>> {
//         unimplemented!()
//     }
// }

#[derive(Table, Debug)]
pub struct CmapHeader<'tbl> {
    buffer: &'tbl [u8],
    version: Ignored<u16>,
    pub num_tables: u16,
}

impl<'a> CmapHeader<'a> {
    pub fn records(&self) -> Result<RecordIter<'a>> {
        let required_size = CmapHeader::size() + self.num_tables as usize * EncodingRecord::size();

        if self.buffer.len() < required_size {
            return Err(Error::UnexpectedEof);
        }

        Ok(RecordIter {
               inherited: self.buffer,
               buffer: &self.buffer[CmapHeader::size()..],
               num_tables: self.num_tables,
               current: 0,
           })
    }
}

#[derive(Debug)]
pub struct RecordIter<'a> {
    inherited: &'a [u8],
    buffer: &'a [u8],
    num_tables: u16,
    current: u16,
}

impl<'a> Iterator for RecordIter<'a> {
    type Item = EncodingRecord<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.num_tables {
            return None;
        }

        self.current += 1;
        let record = match EncodingRecord::parse(self.buffer, self.inherited) {
            Ok(rec) => rec,
            Err(_) => unreachable!(),
        };
        self.buffer = &self.buffer[EncodingRecord::size()..];
        Some(record)
    }

    fn count(self) -> usize {
        (self.num_tables - self.current) as usize
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = (self.num_tables - self.current) as usize;
        (count, Some(count))
    }
}

impl<'a> ExactSizeIterator for RecordIter<'a> {}

#[derive(Debug)]
pub struct EncodingRecord<'tbl> {
    buffer: &'tbl [u8],
    pub platform: u16,
    pub encoding: u16,
    offset: u32,
}

impl<'tbl> SizedTable for EncodingRecord<'tbl> {
    fn size() -> usize {
        8
    }
}

impl<'tbl> TableInherited<'tbl> for EncodingRecord<'tbl> {
    fn parse(mut buffer: &'tbl [u8], inherited: &'tbl [u8]) -> Result<EncodingRecord<'tbl>> {
        if buffer.len() < Self::size() {
            return Err(Error::UnexpectedEof);
        }

        let platform = buffer.read::<u16>()?;
        let encoding = buffer.read::<u16>()?;
        let offset = buffer.read::<u32>()?;

        Ok(EncodingRecord {
               buffer: inherited,
               platform: platform,
               encoding: encoding,
               offset: offset,
           })
    }
}

impl<'a> EncodingRecord<'a> {
    pub fn get_cmap(&self) -> Result<Cmap<'a>> {
        if self.buffer.len() < self.offset as usize + 4 {
            return Err(Error::UnexpectedEof);
        }

        let (_, mut buffer) = self.buffer.split_at(self.offset as usize);
        let version = buffer.read::<u16>()?;
        let (_, buffer) = buffer.split_at(2); // length

        match version {
            0 => Ok(Cmap::Format0(Format0::parse(buffer)?)),
            4 => Ok(Cmap::Format4(Format4::parse(buffer)?)),
            6 => Ok(Cmap::Format6(Format6::parse(buffer)?)),
            12 => Ok(Cmap::Format12(Format12::parse(buffer)?)),
            _ => panic!(format!("unsupported cmap version: {}", version)),
        }
    }
}

#[derive(Table, Debug)]
pub struct Format0<'tbl> {
    buffer: &'tbl [u8],
    language: u16,
}

#[derive(Debug)]
pub struct Format4<'tbl> {
    language: u16,
    end_count: &'tbl [u8],
    start_count: &'tbl [u8],
    id_delta: &'tbl [u8],
    id_offset: &'tbl [u8],
    glyph_ids: &'tbl [u8],
}

impl<'tbl> Table<'tbl> for Format4<'tbl> {
    fn parse(mut buffer: &'tbl [u8]) -> Result<Format4<'tbl>> {
        // bounds check for the first 5 entries whose sizes
        // are known at compile time.
        if buffer.len() < 10 {
            return Err(Error::UnexpectedEof)
        }

        let language  = buffer.read::<u16>()?;
        let seg_count = buffer.read::<u16>()? as usize;

        // searchRange, entrySelector, rangeshift
        let buffer = buffer.split_at(6).1;

        if buffer.len() < 4 * seg_count + 2 {
            return Err(Error::UnexpectedEof)
        }

        let (end_count, buffer) = buffer.split_at(seg_count);
        let (_, buffer) = buffer.split_at(2);  // reserved pad
        let (start_count, buffer) = buffer.split_at(seg_count);
        let (id_delta, buffer) = buffer.split_at(seg_count);
        let (id_offset, glyph_ids) = buffer.split_at(seg_count);

        Ok(Format4 {
            language,
            end_count,
            start_count,
            id_delta,
            id_offset,
            glyph_ids,
        })
    }
}

impl<'tbl> Format4<'tbl> {
    fn get_glyph_id(&self, codepoint: u32) -> Option<u16> {
        use byteorder::{ByteOrder, BigEndian};
        if codepoint >= 0xFFFE {
            return None
        }

        let codepoint = codepoint as u16;
        let mut idx: usize = 0;
        let mut segcode: u16 = 0;

        while idx <= self.end_count.len() {
            let endcode = BigEndian::read_u16(&self.end_count[idx..]);
            if endcode >= codepoint {
                let startcode = BigEndian::read_u16(&self.start_count[idx..]);
                if startcode <= codepoint {
                    // Found index for containing segment
                    segcode = startcode;
                    break;
                } else {
                    return None
                }
            }

            idx += 2;
        }

        // Check if there is a corresponding id_offset
        let id_offset = BigEndian::read_u16(&self.id_offset[idx..]);
        if id_offset == 0 {
            let cp = codepoint as u16;
            let delta = BigEndian::read_u16(&self.id_delta[idx..]);
            Some(cp.wrapping_add(delta))
        } else {
            // The offset is relative to it's current placement
            // so we will immitate this by subtracting the
            // offset by it's current index.
            let correction = self.id_offset.len() - id_offset as usize;
            let pos = id_offset as usize / 2
                + (codepoint - segcode) as usize
                - correction;

            if self.glyph_ids.len() < pos + 2 {
                return None
            }

            let result = BigEndian::read_u16(&self.glyph_ids[idx..]);
            Some(result)
        }
    }
}

#[derive(Debug)]
pub struct Format6<'tbl> {
    language: u16,
    first_code: u16,
    glyph_id_array: &'tbl [u8],
}

impl<'tbl> Table<'tbl> for Format6<'tbl> {
    fn parse(mut buffer: &'tbl [u8]) -> Result<Format6<'tbl>> {
        if buffer.len() < 6 {
            return Err(Error::UnexpectedEof)
        }

        let language = buffer.read::<u16>()?;
        let first_code = buffer.read::<u16>()?;
        let entry_count = buffer.read::<u16>()?;

        if buffer.len() < 2 * entry_count as usize {
            return Err(Error::UnexpectedEof)
        }

        let (glyph_id_array, _) = buffer.split_at(2 * entry_count as usize);

        Ok(Format6 { language, first_code, glyph_id_array })
    }
}

impl<'tbl> Format6<'tbl> {
    fn get_glyph_id(&self, codepoint: u32) -> Option<u16> {
        let offset = match codepoint.checked_sub(self.first_code as u32) {
            Some(offset) => 2 * offset as usize,
            None => return None,
        };

        if offset + 2 > self.glyph_id_array.len() {
            None
        } else {
            let (_, mut buf) = self.glyph_id_array.split_at(offset);
            let result = buf.read::<u16>().unwrap();
            Some(result)
        }
    }
}

#[derive(Debug)]
pub struct Format12<'tbl> {
    language: u32,
    groups: &'tbl [u8],
}

impl<'tbl> Table<'tbl> for Format12<'tbl> {
    fn parse(mut buffer: &'tbl [u8]) -> Result<Format12<'tbl>> {
        if buffer.len() < 12 {
            return Err(Error::UnexpectedEof)
        }

        let _ /* length */ = buffer.read::<u32>()?;
        let language = buffer.read::<u32>()?;
        let num_groups = buffer.read::<u32>()? as usize;
        let size = SequentialMapGroup::size() * num_groups;

        if buffer.len() < size {
            return Err(Error::UnexpectedEof)
        }

        let (groups, _) = buffer.split_at(size);

        Ok(Format12 { language, groups })
    }
}

impl<'tbl> Format12<'tbl> {
    fn get_glyph_id(&self, codepoint: u32) -> Option<u16> {
        let mut buffer = self.groups;
        // TODO: This should be a binary search. There can be a _lot_
        // of groups here.  Ie: SourceHanSansSC-Regular has 16,490.
        while buffer.len() > SequentialMapGroup::size() {
            let seq_map = buffer.read_table::<SequentialMapGroup>().unwrap();
            if let Some(id) = seq_map.get_glyph_id(codepoint) {
                return Some(id)
            }
        }

        None
    }
}

#[derive(Table, Debug)]
pub struct SequentialMapGroup {
    start_char_code: u32,
    end_char_code: u32,
    start_glyph_id: u32,
}

impl SequentialMapGroup {
    pub fn contains(&self, codepoint: u32) -> bool {
        self.start_char_code <= codepoint
        && codepoint <= self.end_char_code
    }

    pub fn get_glyph_id(&self, codepoint: u32) -> Option<u16> {
        match self.contains(codepoint) {
            true => {
                let diff = codepoint - self.start_glyph_id;
                Some(self.start_glyph_id as u16 + diff as u16)
            },

            false => None,
        }
    }
}

#[test]
fn list_cmaps() {
    use font::Font;
    let buf: Vec<u8> = open_font!(r"data/DroidSerif.ttf");

    let font = Font::from_buffer(&buf).expect("unable to parse font");
    let tbl = font.get_table::<CmapHeader>()
        .expect("Failed to read Cmap Header table");

    assert_eq!(tbl.num_tables, 3);

    let mut records = tbl.records()
        .expect("Failed to generated Cmap Records iter");

    assert_cmap_records!(records,
        (0, 3, 28)  Format: 4,
        (1, 0, 148) Format: 0,
        (3, 1, 28)  Format: 4,
    );
}

#[test]
fn format4() {
    use font::Font;
    let buf: Vec<u8> = open_font!(r"data/DroidSerif.ttf");

    let font = Font::from_buffer(&buf).expect("unable to parse font");
    let tbl = font.get_table::<CmapHeader>()
        .expect("Failed to read Cmap Header table");

    let cmap = tbl.records()
        .expect("Failed to generated Cmap Records iter")
        .next()
        .expect("Failed to parse cmap record header");

    let cmap = match cmap.get_cmap().expect("failed to read cmap record") {
        Cmap::Format4(c) => c,
        _ => panic!("format 4 should be the first record"),
    };

    assert!((b' '..b'~')
        .map(|c| cmap.get_glyph_id(c as u32).unwrap())
        .eq(3..97),
        "cmap lookup failed");
}