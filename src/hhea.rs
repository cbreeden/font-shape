use decode::primitives::{Tag, Offset32, Ignore6};
use decode::Parse;
use decode::{Error, Result};

/// horizontal fonts header table
pub struct Hhea {
    pub version:  Tag,
    pub ascent:   FWord,
    pub descent:  FWord,
    pub line_gap: FWord,
    pub advance_width_max: UFWord,
    pub min_left_side_bearing: FWord,
    pub min_right_side_bearing: FWord,
    pub x_max_extent: FWord,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    // Reserved
    ignore: Ignore8,
    pub metric_data_format: i16,
    pub number_of_h_metrics: i16,
}