use decode::primitives::{Tag, FWord, UFWord, Ignored};
use decode::{Error, Result, SizedTable, Table, Primitive, ReadPrimitive, ReadTable};

#[derive(Debug)]
pub enum Maxp {
    Version05 { num_glyphs: u16 },
    Version1(Version1),
}

impl<'tbl> Table<'tbl> for Maxp {
    fn parse(mut buffer: &[u8]) -> Result<Maxp> {
        let version = buffer.read::<u32>()?;
        match version {
            0x00005000 => {
                let n = buffer.read::<u16>()?;
                Ok(Maxp::Version05 { num_glyphs: n })
            },

            0x00010000 => {
                let tbl = Version1::parse(buffer)?;
                Ok(Maxp::Version1(tbl))
            },

            _ => {
                Err(Error::InvalidData)
            }
        }
    }
}

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
    fn maxp() {
        let buf = open_font!(r"data/Roboto-Regular.ttf");
        let font = Font::from_buffer(&buf).expect("Unable to parse font");
        let tbl = font.get_table::<Maxp>().expect("Unable to read Maxp table");

        if let Maxp::Version1(maxp) = tbl {
            assert_eq!(maxp.num_glyphs, 1294);
            assert_eq!(maxp.max_points, 143);
            assert_eq!(maxp.max_contours, 22);
            assert_eq!(maxp.max_composite_points, 84);
            assert_eq!(maxp.max_composite_contours, 5);
            assert_eq!(maxp.max_zones, 1);
            assert_eq!(maxp.max_twilight_points, 0);
            assert_eq!(maxp.max_storage, 0);
            assert_eq!(maxp.max_function_defs, 14);
            assert_eq!(maxp.max_instruction_defs, 0);
            assert_eq!(maxp.max_stack_elements, 512);
            assert_eq!(maxp.max_size_of_instructions, 548);
            assert_eq!(maxp.max_component_elements, 6);
            assert_eq!(maxp.max_component_depth, 1);
        } else { panic!("Parsed incorrect version for maxp"); }
    }
}