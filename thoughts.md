// Desired APIs:
//
// FontLoading:
//   let f = font::from_buffer(buffer, Encoding::Unicode, PlatformID::Windows);
//   for glyph in f.shape("Text given", [Feature1, Feature2])?;

# Required Tables in OpenType:

- cmap
- head
- hhea
- hmtx
- maxp
- name
- OS/2
- post

# TrueType Tables

- Required (Rasterization Related):
  - glyf  (Glyph data)
  - loca  (Index to location)
- Optional:
  - cvt
  - prep
  - fpgm
  - gasp

# PostScript Outlines (Rasterization Related)

- CFF/CFF2 (outlines)
- VORG (optional for vertical writting?)

# Advanced Typographic Tables

- Math
- BASE/GDEF/GPOS/GSUB/JSTF (Common Layout Tables)
