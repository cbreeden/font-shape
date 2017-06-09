use decode::primitives::{Tag, FWord, UFWord, Reserved};
use decode::{Error, Result, SizedTable, Table, Primitive, ReadPrimitive, ReadTable};

pub struct Header<'tbl> {
    scripts: &'tbl [u8],
    features: &'tbl [u8],
    lookups: &'tbl [u8],
    variations: Option<&'tbl [u8]>,
}

impl<'tbl> Table<'tbl> for Header<'tbl> {
    fn parse(mut buffer: &[u8]) -> Result<MarkGlyphSets> {
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
        let (_, features) = head.split_at(features_offset);
        let (_, lookups) = head.split_at(lookups_offset);

        // minor == 1
        let variations = match minor {
            0 => None,
            1 => {
                let offset = buffer.read::<u32>()? as usize;
                verify!(head, offset);
                head.split_at(offset).1
            }
        };

        Ok(Header { scripts, features, lookups, variations })
    }
}