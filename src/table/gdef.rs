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

        // Version >= 1.3
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

pub struct AttachList<'tbl> {
    coverage: &'tbl [u8],
    attach_point: &'tbl [u8],
}

impl<'tbl> Table<'tbl> for AttachList<'tbl> {
    fn parse(mut buffer: &[u8]) -> Result<AttachList> {
        if buffer.len() < 4 {
            return Err(Error::UnexpectedEof)
        }

        let head = buffer;
        let coverage_offset = buffer.read::<u16>()?;
        let glyph_count_size = 2 * buffer.read::<u16>()? as usize;

        if buffer.len() < coverage_offset as usize {
            return Err(Error::UnexpectedEof)
        }

        let (_, coverage) = head.split_at(coverage_offset as usize);

        if buffer.len() < glyph_count_size {
            return Err(Error::UnexpectedEof)
        }

        let (attach_point, _) = buffer.split_at(glyph_count_size);

        Ok(AttachList { coverage, attach_point })
    }
}

pub struct AttachPoint<'tbl> {
    point_index: &'tbl [u8],
}

impl<'tbl> Table<'tbl> for AttachPoint<'tbl> {
    fn parse(mut buffer: &[u8]) -> Result<AttachPoint> {
        let point_count = 2 * buffer.read::<u16>()? as usize;

        if buffer.len() < point_count {
            return Err(Error::UnexpectedEof)
        }

        let (_, point_index) = buffer.split_at(point_count);

        Ok(AttachPoint { point_index })
    }
}

pub struct LigCaretList<'tbl> {
    coverage: &'tbl [u8],
    lig_glyph: &'tbl [u8],
}

impl<'tbl> Table<'tbl> for LigCaretList<'tbl> {
    fn parse(mut buffer: &[u8]) -> Result<LigCaretList> {
        if buffer.len() < 4 {
            return Err(Error::UnexpectedEof)
        }

        let head = buffer;
        let coverage_offset = buffer.read::<u16>()?;
        let glyph_count_size = 2 * buffer.read::<u16>()? as usize;

        if buffer.len() < coverage_offset as usize {
            return Err(Error::UnexpectedEof)
        }

        let (_, coverage) = head.split_at(coverage_offset as usize);

        if buffer.len() < glyph_count_size {
            return Err(Error::UnexpectedEof)
        }

        let (lig_glyph, _) = buffer.split_at(glyph_count_size);

        Ok(LigCaretList { coverage, lig_glyph })
    }
}

enum CaretValue<'tbl> {
    Coordinate(u16),
    ContourPoint(u16),
    DeviceTable {
        value: u16,
        device_table: &'tbl [u8],
    },
}

impl<'tbl> Table<'tbl> for CaretValue<'tbl> {
    fn parse(mut buffer: &[u8]) -> Result<CaretValue> {
        let head = buffer;
        let format = buffer.read::<u16>()?;
        let value = buffer.read::<u16>()?;

        Ok(match format {
            1 => CaretValue::Coordinate(value),
            2 => CaretValue::ContourPoint(value),
            3 => {
                if head.len() < value as usize {
                    return Err(Error::UnexpectedEof)
                }
                let (_, device_table) = buffer.split_at(value as usize);
                CaretValue::DeviceTable { value, device_table }
            }
            _ => return Err(Error::InvalidData)
        })
    }
}

pub struct MarkGlyphSets<'tbl> {
    coverage: &'tbl [u8],
}

impl<'tbl> Table<'tbl> for MarkGlyphSets<'tbl> {
    fn parse(mut buffer: &[u8]) -> Result<MarkGlyphSets> {
        required_len!(buffer, 4);
        let format = buffer.read::<u16>()?;
        let coverage_size = 4 * buffer.read::<u16>()? as usize;
        verify!(format == 1);
        required_len!(buffer, coverage_size);
        let (coverage, _) = buffer.split_at(coverage_size);

        Ok(MarkGlyphSets { coverage })
    }
}

// TODO: Item Variation Store Table, ref: https://www.microsoft.com/typography/otspec/otvaroverview.htm