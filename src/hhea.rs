use decode::primitives::{Tag, FWord, UFWord, Reserved};
use decode::Table;
use decode::StaticSize;
use decode::{Error, Result};

/// horizontal fonts header table
#[derive(Debug, Table)]
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
    _reserve1: Reserved<u32>,
    _reserve2: Reserved<u32>,
    pub metric_data_format: i16,
    pub number_of_h_metrics: i16,
}