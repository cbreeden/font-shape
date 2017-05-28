use decode::primitives::{Tag, Fixed, FWord, UFWord, Reserved};
use decode::{Error, Result, SizedTable, Table, Primitive, ReadPrimitive, ReadTable};

#[derive(Debug, Table)]
pub struct Post {
    pub version: u32,
    pub italic_ange: Fixed,
    pub underline_position: FWord,
    pub underline_thickness: FWord,
    pub is_fixed_pitch: u32,
    pub min_mem_type_42: u32,
    pub max_mem_type_42: u32,
    pub min_mem_type_1: u32,
    pub max_mem_type_1: u32,
}

// TODO:
// Version 2
// Version 2.5
// Version 4.0