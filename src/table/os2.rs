use decode::primitives::{Tag, FWord, UFWord, Reserved};
use decode::Table;
use decode::StaticSize;
use decode::{Error, Result};

#[derive(Debug, Table)]
pub struct Os2 {
    pub version: u16, // 0x005
    pub x_avg_char_width: i16,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: u16,
    pub y_subscript_x_size: i16,
    pub y_subscript_y_size: i16,
    pub y_subscript_x_offset: i16,
    pub y_subscript_y_offset: i16,
    pub y_superscript_x_size: i16,
    pub y_superscript_y_size: i16,
    pub y_superscript_x_offset: i16,
    pub y_superscript_y_offset: i16,
    pub y_strikeout_size: i16,
    pub y_strikeout_position: i16,
    pub s_family_class: i16,
    pub panose: Panrose,
    pub unicode_range_1: u32,
    pub unicode_range_2: u32,
    pub unicode_range_3: u32,
    pub unicode_range_4: u32,
    pub vend_id: Tag,
    pub font_selection: u16,
    pub first_char_index: u16,
    pub last_char_index: u16,
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
    pub win_ascent: u16,
    pub win_descent: u16,
    pub code_page_range_1: u32,
    pub code_page_range_2: u32,
    pub sx_height: i16,
    pub cap_height: i16,
    pub default_char: u16,
    pub break_char: u16,
    pub max_context: u16,
    pub lower_optical_point_size: u16,
    pub upper_optical_point_size: u16,
}

#[derive(Debug, Table)]
pub struct Panrose {
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

#[cfg(test)]
mod test {
    use ::font::Font;
    use ::font::TableRecord;
    use ::decode::primitives::Tag;
    use ::table::os2::Os2;
    use ::decode::Table;

    #[test]
    fn print_tables() {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let file = File::open(r"data/Roboto-Regular.ttf")
            .expect("Unable to open file");

        let mut reader = BufReader::new(file);
        let mut data   = Vec::new();
        reader.read_to_end(&mut data)
            .expect("Error reading file");

        let font = Font::from_buffer(&data)
            .expect("Unable to parse font");

        let TableRecord { offset: offset, .. } = font.tables()
            .map(|tr| tr.unwrap())
            .find(|tr| tr.tag == tag!('O','S','/','2'))
            .unwrap();

        let buf = &data[offset as usize..];
        let (_, maxp) = Os2::parse(buf).unwrap();

        println!("{:#?}", maxp);
    }
}