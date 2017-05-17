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

#[cfg(test)]
mod test {
    use ::font::Font;
    use ::font::TableRecord;
    use ::decode::primitives::Tag;
    use ::table::name::Name;
    use ::decode::Table;

    #[test]
    fn print_names() {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let file = File::open(r"data/OpenSans-Regular.ttf")
            .expect("Unable to open file");

        let mut reader = BufReader::new(file);
        let mut data   = Vec::new();
        reader.read_to_end(&mut data)
            .expect("Error reading file");

        let font = Font::from_buffer(&data)
            .expect("Unable to parse font");

        let TableRecord { offset, .. } = font.tables()
            .map(|tr| tr.unwrap())
            .find(|tr| tr.tag == tag!('n','a','m','e'))
            .unwrap();

        let name_buf = &data[offset as usize..];
        let (_, tbl) = Name::parse(name_buf).unwrap();

        println!("{:#?}", tbl);

        for name in tbl.names(name_buf).unwrap() {
            println!("{:?}", name);
            let start = name.offset as usize;
            let end = start + name.length as usize;
            let s = &name_buf[start..end];

            let s = String::from_utf8_lossy(s);
            println!("Name: {}", s);
        }
    }
}