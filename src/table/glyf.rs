use decode::{Error, Result, SizedTable, Table, TableInherited, Primitive, ReadPrimitive, ReadTable};
use decode::primitives::Ignored;

// TODO: It's unclear if this is needed.
// we might need it for SVG font renderering, but that's probably
// better off in a separate crate.

#[derive(Table, Debug)]
struct GlyphHeader {
	num_of_contours: i16,
	x_min: i16,
	y_min: i16,
	x_max: i16,
	y_max: i16,
}

struct GlyphDescription<'tbl> {
	end_pts_of_contours: &'tbl [u8],
	instructions: &'tbl [u8],
	flags: &'tbl [u8],
	x_coordinates: &'tbl [u8],
	y_coordinates: &'tbl [u8],
}

bitflags! {
    pub struct GlyphFlag: u8 {
		const ON_CURVE			= 1 << 0;
		const X_SHORT_VECTOR	= 1 << 1;
		const Y_SHORT_VECTOR	= 1 << 2;
		const REPEAT			= 1 << 3;
		const X_IS_SAME			= 1 << 4;
		const POSITIVE_X 		= 1 << 4;
		const Y_IS_SAME			= 1 << 5;
		const POSITIVE_Y		= 1 << 5;
	}
}

// struct CompositeDescription

bitflags! {
	pub struct CompositeFlags: u16 {
		const ARG_1_AND_2_ARE_WORDS		= 1 << 0;
		const ARGS_ARE_XY_VALUES		= 1 << 1;
		const ROUND_XY_TO_GRID			= 1 << 2;
		const WE_HAVE_A_SCALE			= 1 << 3;
		const RESERVED					= 1 << 4;
		const MORE_COMPONENTS			= 1 << 5;
		const WE_HAVE_AN_X_AND_Y_SCALE	= 1 << 6;
		const WE_HAVE_A_TWO_BY_TWO		= 1 << 7;
		const WE_HAVE_INSTRUCTIONS		= 1 << 8;
		const USE_MY_METRICS			= 1 << 9;
		const OVERLAP_COMPOUND			= 1 << 10;
		const SCALED_COMPONENT_OFFSET	= 1 << 11;
		const UNSCALED_COMPONENT_OFFSET	= 1 << 12;
	}
}