// Currently only accept utf8 input, and as such we only
// parse cmaps which work with utf8 encodings.
// How could we handle other encodings?  Is this desired?

// Initializing a FontMeta struct
// - ensures a single font face (ttcf)
// - parses whether you have an OpenType or TrueType font
// - parses the font offset table, and ensures the following are present:
//   - cmap / [hhea|vhea] / ... determine required fields.

let meta = match font_meta::from_str(&data)
    .expect("unable to load font_meta data");

Methods avialable after initalization:
 - get_glyph_id(ch: char) -> GlyphId
 - get_glyph_ids(in: &str) -> GlyphIdIterator
 - get_horz_extenst() -> HorzExtents {ascender: EM, descender: EM, line_gap: EM}
 - get_glyph_extents(ch: char) -> GlyphExtents {
        x_breaing: EM,
        y_bearing: EM,
        width:     EM,
        height:    EM,
    }
    // Note: Check glyf table, fallback to cbdt table.
 -


 # PDF Requirements.

 BT            % Begin a text object
   /F13 12 Tf  % Set font and size. F13 = Helvetica, size 12 pt
   288 720 Td  % Set position, 288 pixels left, 720 pixels from up
   (ABC)   Tj  % Paint glyphs of string
ET

# Font Types in PDF
 - Type 3
    - Type 3 dictionary _defines_ the font, as opposed to referering to font programs.
    - Glyphs are defined by streams of PDF graphics operators.
    - No hinting mechanism for low resolutions.
    - Required dictionary entires:
        - \Type \Font
        - \Subtype \Type3
        - \Name % Determine naming conventions for font (pg 413)
        - \FontBBox %
        - \FontMatrix % Define coordinate system [0.001 0 0 0.001 0 0] for 1000 units per em.
        - \CharProcs << \CharacterName stream PDF OPS endstreams >>
        - \Encoding % Section 5.5.5 for character encoding
        - \FirstChar
        - \LastChar
        - \Widths
        - \FontDescriptor % must be an indirect reference.
        - \ToUnicode % A reverse CMap
