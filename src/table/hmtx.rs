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

        if 4 * glyph_id < self.h_metrics.len() {
            let (_, mut record) = self.h_metrics.split_at(4 * glyph_id);
            return record.read_table::<HorizontalMetricRecord>().ok();
        }

        let diff = glyph_id - self.h_metrics.len() / 4;
        if 2 * diff < self.left_side_bearing.len() {
            let (_, mut record) = self.left_side_bearing.split_at(2 * diff);
            record.read::<i16>().ok().map(|lsb|
                HorizontalMetricRecord {
                    advance_width: self.default_advance,
                    lsb: lsb,
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

#[test]
fn horizontal_metrics() {
    use font::Font;
    use table::cmap::CmapHeader;
    let buf: Vec<u8> = open_font!(r"data/DroidSerif.ttf");
    let font = Font::from_buffer(&buf).expect("unable to parse font");

    let hmtx = font.get_table_hmtx()
        .expect("unable to parse hmtx table.");

    macro_rules! assert_eq_hmtx {
        ($(glyph_id: $id:expr, advance: $adv:expr, lsb: $lsb:expr),* $(,)*) => (
            $({
                let c_mtx = hmtx.get_record($id)
                    .expect(&format!("unable to find metrics for '{}'", $id));

                assert_eq!($adv, c_mtx.advance_width, "non-equal advance");
                assert_eq!($lsb, c_mtx.lsb, "non-equal lsb");
            })*
        )
    }

    assert_eq_hmtx! {
	    glyph_id: 36, advance: 1444, lsb: 0, // 'A'
        glyph_id: 37, advance: 1339, lsb: 78, // 'B'
        glyph_id: 38, advance: 1257, lsb: 117, // 'C'
        glyph_id: 39, advance: 1489, lsb: 78, // 'D'
        glyph_id: 40, advance: 1276, lsb: 78, // 'E'
        glyph_id: 41, advance: 1208, lsb: 78, // 'F'
        glyph_id: 42, advance: 1462, lsb: 117, // 'G'
        glyph_id: 43, advance: 1624, lsb: 78, // 'H'
        glyph_id: 44, advance: 752, lsb: 78, // 'I'
        glyph_id: 45, advance: 731, lsb: -23, // 'J'
        glyph_id: 46, advance: 1434, lsb: 78, // 'K'
        glyph_id: 47, advance: 1276, lsb: 78, // 'L'
        glyph_id: 48, advance: 1921, lsb: 78, // 'M'
        glyph_id: 49, advance: 1563, lsb: 78, // 'N'
        glyph_id: 50, advance: 1520, lsb: 115, // 'O'
        glyph_id: 51, advance: 1237, lsb: 78, // 'P'
        glyph_id: 52, advance: 1520, lsb: 115, // 'Q'
        glyph_id: 53, advance: 1343, lsb: 78, // 'R'
        glyph_id: 54, advance: 1114, lsb: 98, // 'S'
        glyph_id: 55, advance: 1255, lsb: 41, // 'T'
        glyph_id: 56, advance: 1468, lsb: 31, // 'U'
        glyph_id: 57, advance: 1382, lsb: 0, // 'V'
        glyph_id: 58, advance: 2144, lsb: 18, // 'W'
        glyph_id: 59, advance: 1352, lsb: 16, // 'X'
        glyph_id: 60, advance: 1280, lsb: -10, // 'Y'
        glyph_id: 61, advance: 1212, lsb: 74, // 'Z'
    };
}