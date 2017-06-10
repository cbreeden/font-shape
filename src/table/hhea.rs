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
    pub number_of_h_metrics: u16,
}

#[cfg(test)]
mod test {
    use ::font::Font;
    use ::font::TableRecord;
    use ::decode::primitives::{Tag, FWord, UFWord};
    use ::table::hhea::Hhea;
    use ::decode::Table;

    #[test]
    fn print_tables() {
        let buf = open_font!("data/OpenSans-Regular.ttf");
        let font = Font::from_buffer(&buf).expect("Unable to parse font");
        let tbl = font.get_table::<Hhea>().expect("unable to read hhea table");

        assert_eq!(tbl.major_version, 1);
        assert_eq!(tbl.minor_version, 0);
        assert_eq!(tbl.ascent, FWord(2189));
        assert_eq!(tbl.descent, FWord(-600));
        assert_eq!(tbl.line_gap, FWord(0));
        assert_eq!(tbl.advance_width_max, UFWord(2476));
        assert_eq!(tbl.min_left_side_bearing, FWord(-1126));
        assert_eq!(tbl.min_right_side_bearing, FWord(-389));
        assert_eq!(tbl.x_max_extent, FWord(2466));
        assert_eq!(tbl.caret_slope_rise, 1);
        assert_eq!(tbl.caret_slope_run, 0);
        assert_eq!(tbl.caret_offset, 0);
        assert_eq!(tbl.metric_data_format, 0);
        assert_eq!(tbl.number_of_h_metrics, 931);
    }
}