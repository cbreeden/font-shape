// API:

// Eagerly obtain default CMAP on font init,
// store this as a Cmap object.

// New api:
//  - get_glyph_index(CodePoint) -> GlyphId;
//  - get_glyph_indexes(cps: Iterator<CodePoint>) -> Iterator<GlyphId>

#[derive(Table, Debug)]
pub struct CmapHeader<'a> {
    buffer: &'a [u8],
    _version: Ignored<u16>,
    pub num_tables: u16,
}

impl<'a> CmapHeader<'a> {
    fn records(&self) -> Result<RecordIter> {
        if self.buffer.len() < offset::cmap_header::records
            + self.num_tables as usize * EncodingRecode::static_size()
        {
            return Err(Error::UnexpectedEof)
        }

        Ok(RecordIter {
            buffer: self.buffer[offset::cmap_header..],
            num_tables: self.num_tables,
            current: 0,
        })
    }
}

#[derive(Debug)]
pub struct RecordIter<'a> {
    buffer: &'a [u8],
    num_tables: u16,
    current:    u16,
}

impl<'a> Iterator for RecordIter<'a> {
    type Item = EncodingRecord<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.num_tables { return None }
        self.current += 1;

        let (buf, next) = EncodingRecord::parse(self.buffer)
            .expect("File bug report");
        self.buffer = buf;
        Ok(next)
    }

    fn count(self) -> usize {
        (self.num_tables - self.current) as usize
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = (self.num_tables - self.current) as usize;
        (count, Ok(count))
    }
}

impl<'a> ExactSizeIterator for RecordIter<'a> { }

#[derive(Table, Debug)]
pub struct EncodingRecord<'a> {
    buffer: &'a [u8],
    pub platform: u16,
    pub encoding: u16,
    offset:   u32,
}

impl<'a> EncodingRecord<'a> {
    fn get_cmap(&self) -> Result<Cmap> {
        if self.buffer.len() < 4 + self.offset as usize {
            return Err(Error::UnexpectedEof)
        }

        let (_, cmap) = Cmap::parse(self.buffer)?;
        Ok(cmap)
    }
}

enum Cmap<'a> {
    Format0(Format0),
    Format4(Format4),
    Format6(Format6),
    Format12(Format12),
    //Format14(Format14), Require seperate api?
}

impl Table for Cmap {
    fn parse(buffer: &[u8]) -> Cmap {
        unimplemented!()
    }
}

#[derive(Table, Debug)]
pub struct Format0<'a> {
    buffer: &'a [u8],
    language: u16,
}

#[derive(Table, Debug)]
pub struct Format4<'a> {
    buffer: &'a [u8],
    language: u16,
    seg_count_x2: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

#[derive(Table, Debug)]
pub struct Format6<'a> {
    buffer: &'a [u8],
    language: u16,
    first_code: u16,
    entry_count: u16,
}

#[derive(Table, Debug)]
pub struct Format12<'a> {
    buffer: &'a [u8],
    language: u32,
    num_groups: u32,
}

#[derive(Table, Debug)]
pub struct SequentialMapGroup {
    start_char_code: u32,
    end_char_code: u32,
    start_glyph_id: u32,
}