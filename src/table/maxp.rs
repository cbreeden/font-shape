use decode::primitives::{Tag, FWord, UFWord, Reserved};
use decode::Table;
use decode::StaticSize;
use decode::{Error, Result};

#[derive(Debug)]
pub enum Maxp {
    Version05 { num_glyphs: u16 },
    Version1(Version1),
}

static_size!(Maxp = 4);
versioned_table!(Maxp,
    u32 => |buf, v| {
        match v {
            0x00005000 => {
                let (buf, n) = u16::parse(buf)?;
                (buf, Maxp::Version05 { num_glyphs: n })
            },
            0x00010000 => {
                let (buf, tbl) = Version1::parse(buf)?;
                (buf, Maxp::Version1(tbl))
            },
            _ => return Err(Error::InvalidData),
        }
    }
);

#[derive(Debug, Table)]
pub struct Version1 {
    pub num_glyphs: u16,
    pub max_points: u16,
    pub max_contours: u16,
    pub max_composite_points: u16,
    pub max_composite_contours: u16,
    pub max_zones: u16,
    pub max_twilight_points: u16,
    pub max_storage: u16,
    pub max_function_defs: u16,
    pub max_instruction_defs: u16,
    pub max_stack_elements: u16,
    pub max_size_of_instructions: u16,
    pub max_component_elements: u16,
    pub max_component_depth: u16,
}

#[cfg(test)]
mod test {
    use ::font::Font;
    use ::font::TableRecord;
    use ::decode::primitives::Tag;
    use ::table::maxp::Maxp;
    use ::decode::Table;

    #[test]
    fn print_tables() {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let file = File::open(r"data/Roboto-Regular.ttf")
            .expect("Unable to open file");

        let mut reader = BufReader::new(file);
        let mut data   = Vec::new();
        reader.read_to_end(&mut data)
            .expect("Error reading file");

        let font = Font::from_buffer(&data)
            .expect("Unable to parse font");

        let TableRecord { offset: offset, .. } = font.tables()
            .map(|tr| tr.unwrap())
            .find(|tr| tr.tag == tag!('m','a','x','p'))
            .unwrap();

        let buf = &data[offset as usize..];
        let (_, maxp) = Maxp::parse(buf).unwrap();

        println!("{:#?}", maxp);
    }
}