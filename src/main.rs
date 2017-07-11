#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate derive_more;
extern crate byteorder;
#[macro_use]
extern crate table_derive;

#[macro_use]
pub mod util;
pub mod font;
pub mod decode;
pub mod table;

use table::cmap::Cmap;
use table::cmap::Format4;
#[inline(never)]
fn get_glpf_indx(cmap: &Format4, cp: u32) -> Option<u16> {
    cmap.get_glyph_id(cp)
}

fn main() {
    use font::Font;
    use table::cmap::CmapHeader;
    let buf: Vec<u8> = open_font!(r"data/DroidSerif.ttf");

    let font = Font::from_buffer(&buf).expect("unable to parse font");
    let tbl = font.get_table::<CmapHeader>()
        .expect("Failed to read Cmap Header table");

    let cmap = tbl.records()
        .expect("Failed to generated Cmap Records iter")
        .next()
        .unwrap();

    let cmap = match cmap.get_cmap().unwrap() {
        Cmap::Format4(c) => c,
        _ => panic!(),
    };

    for idx in b'A'..b'z' {
        let idx = get_glpf_indx(&cmap, idx as u32);
        println!("{:?}", idx);
    }
}