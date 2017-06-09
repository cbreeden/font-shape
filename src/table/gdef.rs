use decode::{Error, Result, SizedTable, Table, TableInherited, Primitive, ReadPrimitive, ReadTable};
use decode::primitives::Ignored;

#[derive(Debug)]
pub struct Header<'tbl> {
    pub glyph_class_def: Option<&'tbl [u8]>,
    pub attach_list: Option<&'tbl [u8]>,
    pub lig_caret_list: Option<&'tbl [u8]>,
    pub mark_attach_class_def: Option<&'tbl [u8]>,
    pub mark_glyph_sets_def: Option<&'tbl [u8]>,
    pub item_var_store: Option<&'tbl [u8]>,
}

macro_rules! offset_maybe_null {
    ($head:expr, $buffer:expr) => ({
        let offset = $buffer.read::<u16>()?;
        match offset {
            0 => None,
            x if $head.len() < x as usize => return Err(Error::UnexpectedEof),
            x => {
                let (_, tbl) = $head.split_at(x as usize);
                Some(tbl)
            }
        }
    })
}

impl<'tbl> Table<'tbl> for Header<'tbl> {
    fn parse(mut buffer: &[u8]) -> Result<Header> {
        if buffer.len() < 12 /* Version 1.0 size */ {
            return Err(Error::UnexpectedEof)
        }

        let head = buffer;
        let _ /* major */ = buffer.read::<u16>()?;
        let minor = buffer.read::<u16>()?;

        // Version >= 1.0
        let glyph_class_def = offset_maybe_null!(head, buffer);
        let attach_list = offset_maybe_null!(head, buffer);
        let lig_caret_list = offset_maybe_null!(head, buffer);
        let mark_attach_class_def = offset_maybe_null!(head, buffer);

        // Version >= 1.2
        let mark_glyph_sets_def = if minor >= 2 {
            if buffer.len() < 2 {
                return Err(Error::UnexpectedEof)
            }
            offset_maybe_null!(head, buffer)
        } else { None };

        let item_var_store = if minor >= 3 {
            if buffer.len() < 4 {
                return Err(Error::UnexpectedEof)
            }
            offset_maybe_null!(head, buffer)
        } else { None };

        Ok(Header {
            glyph_class_def,
            attach_list,
            lig_caret_list,
            mark_attach_class_def,
            mark_glyph_sets_def,
            item_var_store,
        })
    }
}