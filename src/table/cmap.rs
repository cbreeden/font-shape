// API:

// Eagerly obtain default CMAP on font init,
// store this as a Cmap object.

// New api:
//  - get_glyph_index(CodePoint) -> GlyphId;
//  - get_glyph_indexes(cps: Iterator<CodePoint>) -> Iterator<GlyphId>

#[derive(Table, Debug)]
pub struct CmapHeader {
    pub version: u16,
    pub num_tables: u16,
    #[array(length = "num_tables")]
    pub records: Array<EncodingRecord>,
}

pub struct Array<T> {

}

pub trait Table {
    type Target;

    fn parse<T: Read + Seek>(rdr: T) -> ParseResult<T, Self::Target>;
}

pub struct ParseResult<'rdr, R + 'rdr, T> {
    buffer: R,
    table:  T,
}

// font.cmap()
//     .get_records()
//     .into_inner();

// Expands to:
impl Cmap {
    pub fn get_records(&self, )
}