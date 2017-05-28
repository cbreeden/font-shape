use decode::primitives::{Tag, FWord, UFWord, Ignored};
use decode::{Error, Result, SizedTable, Table, Primitive, ReadPrimitive, ReadTable};

/// horizontal fonts header table
#[derive(Debug, Table)]
pub struct Hhea {
    pub major_version: u16,
    pub minor_version: u16,
    pub ascent: FWord,
    pub descent: FWord,
    pub line_gap: FWord,
    pub advance_width_max: UFWord,
    pub min_left_side_bearing: FWord,
    pub min_right_side_bearing: FWord,
    pub x_max_extent: FWord,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    _reserved1: Ignored<u16>,
    _reserved2: Ignored<u16>,
    _reserved3: Ignored<u16>,
    _reserved4: Ignored<u16>,
    pub metric_data_format: i16,
    pub number_of_h_metrics: i16,
}

#[cfg(test)]
mod test {
    use ::font::Font;
    use ::font::TableRecord;
    use ::decode::primitives::Tag;
    use ::table::hhea::Hhea;
    use ::decode::Table;

    #[test]
    fn print_tables() {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let file = open_font!("data/OpenSans-Regular.ttf");

        let font = Font::from_buffer(&file)
            .expect("Unable to parse font");

        let TableRecord { offset: offset, .. } = font.tables()
            .map(|tr| tr.unwrap())
            .find(|tr| tr.tag == Tag(*b"head"))
            .unwrap();

        let buf = &data[offset as usize..];
        let (_, hhea) = Hhea::parse(buf).unwrap();

        println!("{:#?}", hhea);
    }
}