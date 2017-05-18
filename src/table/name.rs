use decode::primitives::{Tag, Fixed, FWord, UFWord, Reserved};
use decode::Table;
use decode::StaticSize;
use decode::{Error, Result};

use std::fmt::Display;

// For historical reasons, it is strongly recommended that
// the name table of all fonts include the Maciontosh platform
#[derive(Debug, Table)]
pub struct Name {
    pub format: u16,
    pub count: u16,
    pub offset: u16,
}

#[derive(Debug, Table)]
pub struct NameRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    pub length: u16,
    pub offset: u16,
}

impl Name {
    pub fn names<'b>(&self, buf: &'b [u8]) -> Result<NameIter<'b>> {
        let required = Self::static_size()
            + NameRecord::static_size() * self.count as usize;

        if buf.len() < required {
            return Err(Error::UnexpectedEof)
        }

        let buf = &buf[Self::static_size()..];

        Ok(NameIter {
            buf: buf,
            n: 0,
            cap: self.count as usize,
        })
    }
}

pub struct NameIter<'a> {
    buf: &'a [u8],
    n:   usize,
    cap: usize,
}

impl<'a> Iterator for NameIter<'a> {
    type Item = NameRecord;

    fn next(&mut self) -> Option<NameRecord> {
        if self.n >= self.cap {
            return None
        }

        let (buf, name) = NameRecord::parse(self.buf)
            .expect("Fatal error: Please report!");

        self.buf = buf;
        self.n += 1;
        Some(name)
    }
}

// Require conversions?
// EnumPrimitive, EnumDisplay (PascalCase -> "Pascal Case")
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NameId {
    Copyright = 0,
    Family = 1,
    Subfamily = 2,
    Identifier = 3,
    FullName = 4,
    Version = 5,
    PostscriptName = 6,
    Trademark = 7,
    Manufacturer = 8,
    Designer = 9,
    Description = 10,
    VenderUrl = 11,
    DesignerUrl = 12,
    License = 13,
    LicenseUrl = 14,
    TypographicFamily = 16,
    TypographicSubfamily = 17,
    CompatibleFull = 18,
    SampleText = 19,
    PostscriptCid = 20,
    WwsFamily = 21,
    WwsSubfamily = 22,
    LightBackgroundPalette = 23,
    DarkBackgroundPalette = 24,
    VariationsPostScriptNamePrefix = 25,
}

// Derived from http://www.unicode.org/Public/MAPPINGS/VENDORS/APPLE/ROMAN.TXT
static MAC_ROMAN: [char; 128] = [
    'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â',
    'ä', 'ã', 'å', 'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì',
    'î', 'ï', 'ñ', 'ó', 'ò', 'ô', 'ö', 'õ', 'ú', 'ù',
    'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶', 'ß',
    '®', '©', '™', '´', '¨', '≠', 'Æ', 'Ø', '∞', '±',
    '≤', '≥', '¥', 'µ', '∂', '∑', '∏', 'π', '∫', 'ª',
    'º', 'Ω', 'æ', 'ø', '¿', '¡', '¬', '√', 'ƒ', '≈',
    '∆', '«', '»', '…', ' ', 'À', 'Ã', 'Õ', 'Œ', 'œ',
    '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ',
    '⁄', '€', '‹', '›', 'ﬁ', 'ﬂ', '‡', '·', '‚', '„',
    '‰', 'Â', 'Ê', 'Á', 'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì',
    'Ó', 'Ô', '', 'Ò', 'Ú', 'Û', 'Ù', 'ı', 'ˆ', '˜',
    '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ'
];

fn decode_mac_roman(buf: &[u8]) -> String {
    buf.iter()
        .map(|&c| {
            if c <= 128 {
                c as char
            } else {
                MAC_ROMAN[c as usize - 128]
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use ::font::Font;
    use ::font::TableRecord;
    use ::decode::primitives::Tag;
    use ::table::name::Name;
    use ::decode::Table;
    use super::decode_mac_roman;

    #[test]
    fn print_names() {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let file = File::open(r"data/OpenSans-Regular.ttf")
            .expect("unable to open file");

        let mut reader = BufReader::new(file);
        let mut data   = Vec::new();
        reader.read_to_end(&mut data)
            .expect("error reading font");

        let font = Font::from_buffer(&data)
            .expect("unable to parse font");

        let offset = font.get_table_offset(Tag(*b"name"))
            .expect("unable to find 'name' table");

        let name_buf = &data[offset as usize..];
        let (_, tbl) = Name::parse(name_buf).unwrap();

        println!("{:#?}", tbl);

        for name in tbl.names(name_buf).unwrap() {
            println!("{:?}", name);

            let start = tbl.offset as usize
                + name.offset as usize;
            let end = start + name.length as usize;
            let s = &name_buf[start..end];

            if name.platform_id == 1 && name.encoding_id == 0 {
                let s = decode_mac_roman(s);
                println!("Name: {}", s);
            };
        }
    }
}