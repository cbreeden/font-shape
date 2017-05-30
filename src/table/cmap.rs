use decode::{Error, Result, SizedTable, Table, TableInherited, Primitive, ReadPrimitive, ReadTable};
use decode::primitives::Ignored;

// API:

// Eagerly obtain default CMAP on font init,
// store this as a Cmap object.

// New api:
//  - get_glyph_index(CodePoint) -> GlyphId;
//  - get_glyph_indexes(cps: Iterator<CodePoint>) -> Iterator<GlyphId>

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
}

impl<'tbl> Table<'tbl> for Cmap<'tbl> {
    fn parse(buffer: &[u8]) -> Result<Cmap<'tbl>> {
        unimplemented!()
    }
}

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
        // We need to backtrack, since the offset is given relative to CmapHeader
        // Ensure we have enough bytes to jump and read format and length
        if self.buffer.len() < self.offset as usize + 4 {
            return Err(Error::UnexpectedEof);
        }

        let mut buffer = &self.buffer[self.offset as usize..];
        let version = buffer.read::<u16>()?;
        let _ = buffer.read::<Ignored<u16>>()?; // length

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
    //buffer: &'tbl [u8],
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
        let _ /* searchRange */ = buffer.read::<u16>()?;
        let _ /* entrySelector */ = buffer.read::<u16>()?;
        let _ /* rangeShift */ = buffer.read::<u16>()?;

        if buffer.len() < 4 * seg_count + 2 {
            return Err(Error::UnexpectedEof)
        }

        let (end_count, buffer) = buffer.split_at(seg_count);
        let buffer = &buffer[2..];    // reserved pad + 2
        let (start_count, buffer) = buffer.split_at(seg_count);
        let (id_delta, buffer) = buffer.split_at(seg_count);
        let (id_offset, glyph_ids) = buffer.split_at(seg_count);

        Ok(Format4 {
            language: language,
            end_count: end_count,
            start_count: start_count,
            id_delta: id_delta,
            id_offset: id_offset,
            glyph_ids: glyph_ids,
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
        let mut found = false;
        let mut segcode: u16 = 0;

        while idx <= self.end_count.len() {
            let endcode = BigEndian::read_u16(&self.end_count[idx..]);
            if endcode >= codepoint {
                let startcode = BigEndian::read_u16(&self.start_count[idx..]);
                if startcode <= codepoint {
                    // Found index for containing segment
                    // Check if there is a corresponding id_offset
                    found = true;
                    segcode = startcode;
                    break;
                } else {
                    // Failed segment containment
                    return None
                }
            }

            idx += 2;
        }

        // This should be unreachable according to the specs
        if !found {
            return None
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
        .unwrap();

    let cmap = match cmap.get_cmap().unwrap() {
        Cmap::Format4(c) => c,
        _ => panic!(),
    };

    for c in b'A'..b'z' {
        let id = cmap.get_glyph_id(c as u32);
        println!("{:?}: {:?}", ::std::char::from_u32(c as u32), id);
    }
}

// impl<'tbl> Format4<'tbl> {
//     fn get_glyph_index(&self, codepoint: u32) -> u16 {
//         // Perform a binary seach on the ranges.
//         let

//         let mut index: usize = 0;
//         let mut min: usize = 0;
//         let mut max: usize = (self.seg_count_x2 / 2) as usize - 1;
//         let mut found = false;

//         while min <= max {
//             index = ((min + max) / 2) as usize;
//             if codepoint > self.end_chars[index] {
//                 min = index + 1;
//             } else if codepoint < self.start_chars[index] {
//                 max = index - 1;
//             } else {
//                 found = true;
//                 break;
//             }
//         }

//         if !found {
//             return None;
//         }

//         if self.id_offset[index] != 0 {
//             panic!("Non-zero offset unsupported for now.");
//         } else {
//             let idx = (codepoint - self.start_chars[index] + self.id_delta[index]) as usize;
//             Ok(idx)
//         }
//     }
// }

#[derive(Table, Debug)]
pub struct Format6<'tbl> {
    buffer: &'tbl [u8],
    language: u16,
    first_code: u16,
    entry_count: u16,
}

#[derive(Table, Debug)]
pub struct Format12<'tbl> {
    buffer: &'tbl [u8],
    language: u32,
    num_groups: u32,
}

#[derive(Table, Debug)]
pub struct SequentialMapGroup {
    start_char_code: u32,
    end_char_code: u32,
    start_glyph_id: u32,
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