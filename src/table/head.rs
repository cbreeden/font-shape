use decode::primitives::Fixed;
use decode::{Primitive, Table, Error, Result};

pub struct Head<'tbl> {
    buffer: &'tbl [u8],
}

impl<'tbl> Table<'tbl> for Head<'tbl> {
    fn parse(buffer: &'tbl [u8]) -> Result<Head<'tbl>> {
        // TODO: The total size should exported from macro
        if buffer.len() < offsets::glyph_data_format + 2 {
            return Err(Error::UnexpectedEof)
        }

        Ok(Head {
            buffer: buffer
        })
    }
}

impl_offset_table!(Head,
    major_version: u16,
    minor_version: u16,
    font_revision: Fixed,
    check_sum_adjustment: i32,
    magic_number: i32,
    flags: u16,
    units_per_em: u16,
    created: i64,
    modified: i64,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    index_to_loc_format: i16,
    glyph_data_format: i16,
);

#[test]
fn head() {
    use font::Font;

    let buf = open_font!("data/OpenSans-Regular.ttf");
    let font = Font::from_buffer(&buf).expect("Unable to parse font");
    let tbl = font.get_table::<Head>().expect("unable to read Head table");

    assert_offset_table!(tbl,
        major_version: 1,
        minor_version: 0,
        font_revision: Fixed(72090),
        check_sum_adjustment: 566752607,
        magic_number: 1594834165,
        flags: 9,
        units_per_em: 2048,
        created: 3375706507,
        modified: 3387444300,
        x_min: -1126,
        y_min: -555,
        x_max: 2466,
        y_max: 2146,
        mac_style: 0,
        lowest_rec_ppem: 9,
        font_direction_hint: 2,
        index_to_loc_format: 0,
        glyph_data_format: 0,
    );
}