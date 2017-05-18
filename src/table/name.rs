use decode::Table;
use decode::StaticSize;
use decode::{Error, Result};

// API Guidelines:
//
// Names should be accessible from a `Font` instance via a getter method.  This will default to
// finding names using the Mac OS, Roman, encoding scheme.  TODO: Should fallback to the Windows
// english encoding scheme.
//
// Example: `font.get_copyright() -> Result<String>`.
//
// Names should be accessible from a `Font` instance via `(Platform, Encoding, Language)` triple along
// with a NameID.  In this case, the actual encoding should be handled by the user?  Or should this
// capability be gated from a feature.
//
// A name record iterator should be accessible a font instance: `Iterator<Item=NameRecord> for NameIter`.
// Each NameRecord should provide a `.get_name() -> Result<String>` method.  This requires a buffer as well.
// Expected API:
//
// ```
//   font.names().find(|nr| nr.platform_id == platform::Microsoft).unwrap().get_name();
// ```

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

macro_rules! get_name {
    ($($name:ident = $id:expr),*) => (
        $(
        pub fn $name(&self, buf: &[u8]) -> Option<String> {
            let mut names = match self.names(buf) {
                Err(_) => return None,
                Ok(names) => names,
            };

            let rec = 
                match names.find(|nr|
                    nr.platform_id == 1 &&
                    nr.encoding_id == 0 &&
                    nr.language_id == 0 &&
                    nr.name_id == $id) 
                {
                    None => return None,
                    Some(name_record) => name_record,
                };

            let start = self.offset as usize + rec.offset as usize;
            let end = start + rec.length as usize;

            if buf.len() < end {
                return None
            } else {
                let s = &buf[start..end];
                Some(decode_mac_roman(s))
            }
        }
        )*
    )
}


impl Name {
    pub fn names<'b>(&self, buf: &'b [u8]) -> Result<NameIter<'b>> {
        let required = Self::static_size() + NameRecord::static_size() * self.count as usize;

        if buf.len() < required {
            return Err(Error::UnexpectedEof);
        }

        let buf = &buf[Self::static_size()..];

        Ok(NameIter {
               buf: buf,
               n: 0,
               cap: self.count as usize,
           })
    }

    get_name!(get_copyright = 0,
              get_family = 1,
              get_subfamily = 2,
              get_identifier = 3,
              get_full_name = 4,
              get_version = 5,
              get_postscript_name = 6,
              get_trademark = 7,
              get_manufacturer = 8,
              get_designer = 9,
              get_description = 10,
              get_vender_url = 11,
              get_designer_url = 12,
              get_license = 13,
              get_license_url = 14,
              get_typographic_family = 16,
              get_typographic_subfamily = 17,
              get_compatible_full_name = 18,
              get_sample_text = 19,
              get_postscript_cid = 20);
}

pub struct NameIter<'a> {
    buf: &'a [u8],
    n: usize,
    cap: usize,
}

impl<'a> Iterator for NameIter<'a> {
    type Item = NameRecord;

    fn next(&mut self) -> Option<NameRecord> {
        if self.n >= self.cap {
            return None;
        }

        let (buf, name) = NameRecord::parse(self.buf).expect("Fatal error: Please report!");

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
static MAC_ROMAN: [char; 128] =
    ['Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å', 'ç', 'é',
     'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô', 'ö', 'õ', 'ú', 'ù',
     'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶', 'ß', '®', '©', '™', '´', '¨',
     '≠', 'Æ', 'Ø', '∞', '±', '≤', '≥', '¥', 'µ', '∂', '∑', '∏', 'π', '∫',
     'ª', 'º', 'Ω', 'æ', 'ø', '¿', '¡', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…',
     ' ', 'À', 'Ã', 'Õ', 'Œ', 'œ', '–', '—', '“', '”', '‘', '’', '÷', '◊',
     'ÿ', 'Ÿ', '⁄', '€', '‹', '›', 'ﬁ', 'ﬂ', '‡', '·', '‚', '„', '‰', 'Â',
     'Ê', 'Á', 'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', '', 'Ò', 'Ú', 'Û', 'Ù',
     'ı', 'ˆ', '˜', '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ'];

fn decode_mac_roman(buf: &[u8]) -> String {
    buf.iter()
        .map(|&c| if c <= 128 {
                 c as char
             } else {
                 MAC_ROMAN[c as usize - 128]
             })
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use font::Font;
    use decode::primitives::Tag;
    use table::name::Name;
    use decode::Table;

    #[test]
    fn print_names() {
        let font = open_font!(r"data/OpenSans-Regular.ttf");
        let tbl = font.get_name_table().expect("unable to parse name table");

        //let copyright = tbl.get_copyright(name_buf);
        println!("{:?}", copyright);
    }
}