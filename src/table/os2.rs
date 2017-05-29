#![allow(non_snake_case)]
use decode::primitives::{Tag, FWord, UFWord, Ignored};
use decode::{Error, Result, SizedTable, Table, Primitive, ReadPrimitive, ReadTable};

pub struct Os2<'tbl> {
    buffer: &'tbl [u8]
}

impl<'tbl> Table<'tbl> for Os2<'tbl> {
    fn parse(buffer: &'tbl [u8]) -> Result<Os2<'tbl>> {
        if buffer.len() < offsets::us_upper_optical_point_size + 2 {
            return Err(Error::UnexpectedEof)
        }

        Ok(Os2 {
            buffer: buffer
        })
    }
}

#[derive(Debug, Table, PartialEq)]
pub struct Panose {
    pub familty_type: u8,
    pub serif_style: u8,
    pub weight: u8,
    pub proportion: u8,
    pub contrast: u8,
    pub stroke_variation: u8,
    pub arm_style: u8,
    pub letter_form: u8,
    pub midline: u8,
    pub x_height: u8,
}

impl_offset_table!(Os2,
    version: u16,
    x_avg_char_width: i16,
    us_weight_class: u16,
    us_width_class: u16,
    fs_type: u16,
    y_subscript_x_size: i16,
    y_subscript_y_size: i16,
    y_subscript_x_offset: i16,
    y_subscript_y_offset: i16,
    y_superscript_x_size: i16,
    y_superscript_y_size: i16,
    y_superscript_x_offset: i16,
    y_superscript_y_offset: i16,
    y_strikeout_size: i16,
    y_strikeout_position: i16,
    s_family_class: i16,
    panose: Panose,
    ul_unicode_range1: u32,
    ul_unicode_range2: u32,
    ul_unicode_range3: u32,
    ul_unicode_range4: u32,
    ach_vend_id: Tag,
    fs_selection: u16,
    us_first_char_index: u16,
    us_last_char_index: u16,
    s_typo_ascender: i16,
    s_typo_descender: i16,
    s_typo_line_gap: i16,
    us_win_ascent: u16,
    us_win_descent: u16,
    ul_code_page_range1: u32,
    ul_code_page_range2: u32,
    x_height: i16,
    x_cap_height: i16,
    us_default_char: u16,
    us_break_char: u16,
    us_max_context: u16,
    us_lower_optical_point_size: u16,
    us_upper_optical_point_size: u16,
);

#[test]
fn version() {
    use font::Font;

    let buf = open_font!("data/OpenSans-Regular.ttf");
    let font = Font::from_buffer(&buf).expect("Unable to parse font");
    let tbl = font.get_table::<Os2>().expect("unable to read OS/2 table");

    assert_offset_table!(tbl,
        version: 3,
        x_avg_char_width: 1206,
        us_weight_class: 400,
        us_width_class: 5,
        fs_type: 8,
        y_subscript_x_size: 1434,
        y_subscript_y_size: 1331,
        y_subscript_x_offset: 0,
        y_subscript_y_offset: 287,
        y_superscript_x_size: 1434,
        y_superscript_y_size: 1331,
        y_superscript_x_offset: 0,
        y_superscript_y_offset: 977,
        y_strikeout_size: 102,
        y_strikeout_position: 497,
        s_family_class: 2050,
        panose:
            Panose {
                familty_type: 2,
                serif_style: 11,
                weight: 6,
                proportion: 6,
                contrast: 3,
                stroke_variation: 5,
                arm_style: 4,
                letter_form: 2,
                midline: 2,
                x_height: 4
            },
        ul_unicode_range1: 3758097135,
        ul_unicode_range2: 1073750107,
        ul_unicode_range3: 40,
        ul_unicode_range4: 0,
        ach_vend_id: Tag(*b"1ASC"),
        fs_selection: 64,
        us_first_char_index: 32,
        us_last_char_index: 65533,
        s_typo_ascender: 1567,
        s_typo_descender: -492,
        s_typo_line_gap: 132,
        us_win_ascent: 2189,
        us_win_descent: 600,
        ul_code_page_range1: 536871327,
        ul_code_page_range2: 0,
        x_height: 1096,
        x_cap_height: 1462,
        us_default_char: 0,
        us_break_char: 32,
        us_max_context: 3,
        us_lower_optical_point_size: 1229,
        us_upper_optical_point_size: 193,
    );
}