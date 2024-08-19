use std::collections::HashMap;

#[derive(Debug)]
pub struct FontData {
    pub tables: HashMap<String, GlyphTable>,
}

#[derive(Debug)]
pub struct GlyphTable {
    pub tag: Tag,
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
}

type Tag = [char; 4];

#[derive(Debug)]
pub struct GlyphData {
    pub contour_end_indices: Vec<usize>,
    pub points: Vec<GlyphPoint>,
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphPoint {
    pub x: i64,
    pub y: i64,
}
