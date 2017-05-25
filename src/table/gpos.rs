use decode::primitives::{Tag, FWord, UFWord, Reserved};
use decode::Table;
use decode::StaticSize;
use decode::{Error, Result};

enum Version {
    Version1,
    Version11,
}

pub struct Header<'a> {
    buffer: &'a [u8],
    pub version: Version,
    scripts: usize,
    features: usize,
    lookups: usize,
    variations: usize,
}

