use decode::{Error, Result, SizedTable, Table, TableInherited, Primitive, ReadPrimitive, ReadTable};
use decode::primitives::Ignored;

#[derive(Debug)]
pub struct Hmtx<'tbl> {
    h_metrics: &'tbl [u8],
    left_side_bearing: &'tbl [u8],
    default_advance: u16,
}

impl<'tbl> Hmtx<'tbl> {
    /// The number of glyphs are found in the `maxp` table,
    /// and the number of h_glyphs are found in the `hhead` table.
    pub fn parse(buffer: &[u8], num_glyphs: u16, num_h_glyphs: u16) -> Result<Hmtx> {
        let hm_size = HorizontalMetricRecord::size() * num_h_glyphs as usize;
        let lsb_size = 2 * (num_glyphs - num_h_glyphs) as usize;

        required_len!(buffer, hm_size + lsb_size);
        verify!(num_h_glyphs >= 1);

        let (h_metrics, buffer) = buffer.split_at(hm_size);
        let (left_side_bearing, _) = buffer.split_at(lsb_size);

        let (_, mut last_record) = h_metrics
            .split_at(hm_size - HorizontalMetricRecord::size());
        let default_advance = last_record.read_table::<HorizontalMetricRecord>()?
            .advance_width;

        Ok(Hmtx { h_metrics, left_side_bearing, default_advance })
    }

    fn get_record(&self, glyph_id: u16) -> Option<HorizontalMetricRecord> {
        let glyph_id = glyph_id as usize;

        if glyph_id < self.h_metrics.len() / 4 {
            let (_, mut record) = self.h_metrics.split_at(4 * glyph_id);
            let hmtx = record.read_table::<HorizontalMetricRecord>().unwrap();
            Some(hmtx)
        } else if glyph_id < self.h_metrics.len() / 4
                + self.left_side_bearing.len() {
            let offset = glyph_id - self.h_metrics.len() / 4;
            let (_, mut record) = self.left_side_bearing.split_at(2 * offset);
            let lsb = record.read::<i16>().unwrap();

            Some(HorizontalMetricRecord {
                advance_width: self.default_advance,
                lsb: lsb
            })
        } else {
            None
        }
    }

    fn get_advance(&self, glyph_id: u16) -> Option<u16> {
        self.get_record(glyph_id).map(|r| r.advance_width)
    }

    fn get_lsb(&self, glyph_id: u16) -> Option<i16> {
        self.get_record(glyph_id).map(|r| r.lsb)
    }
}

#[derive(Debug, Table)]
pub struct HorizontalMetricRecord {
    pub advance_width: u16,
    pub lsb: i16,
}