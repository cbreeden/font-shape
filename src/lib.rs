#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate derive_more;
extern crate byteorder;
#[macro_use]
extern crate table_derive;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate min_max_macros;

#[macro_use]
pub mod util;
pub mod font;
pub mod decode;
pub mod table;