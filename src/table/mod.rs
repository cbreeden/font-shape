pub mod head;
pub mod hhea;
pub mod maxp;
pub mod os2;
pub mod name;
pub mod cmap;
pub mod glyf;
pub mod gdef;

use decode::primitives::Tag;
use decode::Table;

/// Tagged tables are tables that are accessed from the Font.
pub trait TaggedTable<'tbl>: Table<'tbl> {
    fn tag() -> Tag;
}

macro_rules! impl_tagged_table {
    ($($name:ty => $tag:expr),* $(,)*) => (
        $(
        impl<'tbl> TaggedTable<'tbl> for $name {
            fn tag() -> Tag {
                Tag($tag)
            }
        }
        )*
    )
}

impl_tagged_table!(
    name::Name<'tbl> => *b"name",
    hhea::Hhea => *b"hhea",
    maxp::Maxp => *b"maxp",
    os2::Os2<'tbl> => *b"OS/2",
    head::Head<'tbl> => *b"head",
    cmap::CmapHeader<'tbl> => *b"cmap",
);