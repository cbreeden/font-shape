#![allow(non_snake_case)]
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

pub struct Os2<'tbl> {
    buffer: &'tbl [u8]
}

impl<'tbl> Table<'tbl> for Os2<'tbl> {
    fn parse(buffer: &'tbl [u8]) -> Result<Os2<'tbl>> {
        if buffer.len() < offsets::usUpperOpticalPointSize + 2 {
            return Err(Error::UnexpectedEof)
        }

        Ok(Os2 {
            buffer: buffer
        })
    }
}

macro_rules! impl_os2 {
    ($($fn:ident, $offset:ident: $ret:tt),* $(,)*) => {
        impl<'tbl> Os2<'tbl>  {
            $(
            fn $fn(&self) -> $ret {
                $ret::parse(&self.buffer[offsets::$offset..])
                    .expect("can't fail")
            }
            )*
        }
    }
}

impl_os2! {
    get_version, version: u16,
    get_xAvgCharWidth, xAvgCharWidth: i16,
    get_usWeightClass, usWeightClass: u16,
    get_usWidthClass, usWidthClass: u16,
    get_fsType, fsType: u16,
    get_ySubscriptXSize, ySubscriptXSize: i16,
    get_ySubscriptYSize, ySubscriptYSize: i16,
    get_ySubscriptXOffset, ySubscriptXOffset: i16,
    get_ySubscriptYOffset, ySubscriptYOffset: i16,
    get_ySuperscriptXSize, ySuperscriptXSize: i16,
    get_ySuperscriptYSize, ySuperscriptYSize: i16,
    get_ySuperscriptXOffset, ySuperscriptXOffset: i16,
    get_ySuperscriptYOffset, ySuperscriptYOffset: i16,
    get_yStrikeoutSize, yStrikeoutSize: i16,
    get_yStrikeoutPosition, yStrikeoutPosition: i16,
    get_sFamilyClass, sFamilyClass: i16,
    get_panose, panose: Panose,
    get_ulUnicodeRange1, ulUnicodeRange1: u32,
    get_ulUnicodeRange2, ulUnicodeRange2: u32,
    get_ulUnicodeRange3, ulUnicodeRange3: u32,
    get_ulUnicodeRange4, ulUnicodeRange4: u32,
    get_achVendID, achVendID: Tag,
    get_fsSelection, fsSelection: u16,
    get_usFirstCharIndex, usFirstCharIndex: u16,
    get_usLastCharIndex, usLastCharIndex: u16,
    get_sTypoAscender, sTypoAscender: i16,
    get_sTypoDescender, sTypoDescender: i16,
    get_sTypoLineGap, sTypoLineGap: i16,
    get_usWinAscent, usWinAscent: u16,
    get_usWinDescent, usWinDescent: u16,
    get_ulCodePageRange1, ulCodePageRange1: u32,
    get_ulCodePageRange2, ulCodePageRange2: u32,
    get_xHeight, xHeight: i16,
    get_xCapHeight, xCapHeight: i16,
    get_usDefaultChar, usDefaultChar: u16,
    get_usBreakChar, usBreakChar: u16,
    get_usMaxContext, usMaxContext: u16,
    get_usLowerOpticalPointSize, usLowerOpticalPointSize: u16,
    get_usUpperOpticalPointSize, usUpperOpticalPointSize: u16,
}

#[test]
fn version() {
    macro_rules! print_values {
        ($tbl:expr, $($fn:ident),* $(,)*) => (
            $(
                println!("{}: {:?}", stringify!($fn), $tbl.$fn());
            )*
        )
    }

    use font::Font;

    let buf = open_font!("data/OpenSans-Regular.ttf");
    let font = Font::from_buffer(&buf).expect("Unable to parse font");
    let tbl = font.get_table::<Os2>().expect("unable to read OS/2 table");

    // print_values!(tbl,
    //     get_version,
    //     get_xAvgCharWidth,
    //     get_usWeightClass,
    //     get_usWidthClass,
    //     get_fsType,
    //     get_ySubscriptXSize,
    //     get_ySubscriptYSize,
    //     get_ySubscriptXOffset,
    //     get_ySubscriptYOffset,
    //     get_ySuperscriptXSize,
    //     get_ySuperscriptYSize,
    //     get_ySuperscriptXOffset,
    //     get_ySuperscriptYOffset,
    //     get_yStrikeoutSize,
    //     get_yStrikeoutPosition,
    //     get_sFamilyClass,
    //     get_panose,
    //     get_ulUnicodeRange1,
    //     get_ulUnicodeRange2,
    //     get_ulUnicodeRange3,
    //     get_ulUnicodeRange4,
    //     get_achVendID,
    //     get_fsSelection,
    //     get_usFirstCharIndex,
    //     get_usLastCharIndex,
    //     get_sTypoAscender,
    //     get_sTypoDescender,
    //     get_sTypoLineGap,
    //     get_usWinAscent,
    //     get_usWinDescent,
    //     get_ulCodePageRange1,
    //     get_ulCodePageRange2,
    //     get_xHeight,
    //     get_xCapHeight,
    //     get_usDefaultChar,
    //     get_usBreakChar,
    //     get_usMaxContext,
    //     get_usLowerOpticalPointSize,
    //     get_usUpperOpticalPointSize,
    // );
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