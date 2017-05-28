use decode::primitives::{Tag, FWord, UFWord, Ignored};
use decode::{Error, Result, SizedTable, Table, Primitive, ReadPrimitive, ReadTable};

offsets! {
    version: u16,
    xAvgCharWidth: i16,
    usWeightClass: u16,
    usWidthClass: u16,
    fsType: u16,
    ySubscriptXSize: i16,
    ySubscriptYSize: i16,
    ySubscriptXOffset: i16,
    ySubscriptYOffset: i16,
    ySuperscriptXSize: i16,
    ySuperscriptYSize: i16,
    ySuperscriptXOffset: i16,
    ySuperscriptYOffset: i16,
    yStrikeoutSize: i16,
    yStrikeoutPosition: i16,
    sFamilyClass: i16,
    panose: Panose,
    ulUnicodeRange1: u32,
    ulUnicodeRange2: u32,
    ulUnicodeRange3: u32,
    ulUnicodeRange4: u32,
    achVendID: Tag,
    fsSelection: u16,
    usFirstCharIndex: u16,
    usLastCharIndex: u16,
    sTypoAscender: i16,
    sTypoDescender: i16,
    sTypoLineGap: i16,
    usWinAscent: u16,
    usWinDescent: u16,
    ulCodePageRange1: u32,
    ulCodePageRange2: u32,
    xHeight: i16,
    xCapHeight: i16,
    usDefaultChar: u16,
    usBreakChar: u16,
    usMaxContext: u16,
    usLowerOpticalPointSize: u16,
    usUpperOpticalPointSize: u16,
}

pub struct Os2<'a> {
    inner: &'a [u8]
}

#[derive(Debug, Table)]
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