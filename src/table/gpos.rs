// use decode::primitives::{Tag, FWord, UFWord};
use decode::{Error, Result, SizedTable, Table, Primitive, ReadPrimitive, ReadTable};

pub struct Header<'tbl> {
    scripts: &'tbl [u8],
    features: &'tbl [u8],
    lookups: &'tbl [u8],
    variations: Option<&'tbl [u8]>,
}

impl<'tbl> Table<'tbl> for Header<'tbl> {
    fn parse(mut buffer: &[u8]) -> Result<Header> {
        required_len!(buffer, 10);

        let head = buffer;
        let major = buffer.read::<u16>()?;
        let minor = buffer.read::<u16>()?;

        verify!(major == 1);

        // minor >= 0
        let scripts_offset = buffer.read::<u16>()? as usize;
        let feature_offset = buffer.read::<u16>()? as usize;
        let lookup_offset = buffer.read::<u16>()? as usize;

        required_len!(head,
            max!(scripts_offset, feature_offset, lookup_offset));

        let (_, scripts) = head.split_at(scripts_offset);
        let (_, features) = head.split_at(feature_offset);
        let (_, lookups) = head.split_at(lookup_offset);

        // minor == 1
        let variations = match minor {
            0 => None,
            1 => {
                let offset = buffer.read::<u32>()? as usize;
                required_len!(head, offset);
                Some(head.split_at(offset).1)
            }
            _ => return Err(Error::InvalidData),
        };

        Ok(Header { scripts, features, lookups, variations })
    }
}

bitflags! {
    pub struct ValueFormatFlags: u16 {
        const X_PLACEMENT        = 1 << 0;
        const Y_PLACEMENT        = 1 << 1;
        const X_ADVANCE          = 1 << 2;
        const Y_ADVANCE          = 1 << 3;
        const X_PLACEMENT_DEVICE = 1 << 4;
        const Y_PLACEMENT_DEVICE = 1 << 5;
        const X_ADVANCE_DEVICE   = 1 << 6;
        const Y_ADVANCE_DEVICE   = 1 << 7;
    }
}

pub struct ValueRecord {
    x_placement: Option<i16>,
    y_placement: Option<i16>,
    x_advance: Option<i16>,
    y_advance: Option<i16>,
    // TODO: implement device tables.
    // x_placement_device: Option<&'tbl [u8]>,
    // y_placement_device: Option<&'tbl [u8]>,
    // x_advance_device: Option<&'tbl [u8]>,
    // y_advance_device: Option<&'tbl [u8]>,
}

impl ValueRecord {
    fn parse(mut buffer: &[u8], format: ValueFormatFlags) -> Result<ValueRecord> {
        // TODO: This is innacurate if the ValueRecord is a the tail end of a font
        // and is only partially constructed.  I just don't know of a better way
        // at the moment.
        required_len!(buffer, 8);

        let temp_x_pla = buffer.read::<i16>()?;
        let temp_y_pla = buffer.read::<i16>()?;
        let temp_x_adv = buffer.read::<i16>()?;
        let temp_y_adv = buffer.read::<i16>()?;

        let x_placement = match format.contains(X_PLACEMENT) {
            false => None,
            true => Some(temp_x_pla),
        };

        let y_placement = match format.contains(Y_PLACEMENT) {
            false => None,
            true => Some(temp_y_pla),
        };

        let x_advance = match format.contains(X_ADVANCE) {
            false => None,
            true => Some(temp_x_adv),
        };

        let y_advance = match format.contains(Y_ADVANCE) {
            false => None,
            true => Some(temp_y_adv)
        };

        Ok(ValueRecord { x_placement, y_placement, x_advance, y_advance })
    }
}

pub struct SinglePosFormat1<'tbl> {
    coverage: &'tbl [u8],
    value: ValueRecord,
}

impl<'tbl> Table<'tbl> for SinglePosFormat1<'tbl> {
    fn parse(buffer: &[u8]) -> Result<SinglePosFormat1> {
        unimplemented!()
    }
}