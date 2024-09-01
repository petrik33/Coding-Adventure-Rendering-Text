use std::io;

use enum_map::EnumMap;

#[derive(Debug)]
pub struct FontData {
    pub tables: EnumMap<TableTag, Option<TableData>>,
}

impl FontData {
    pub fn try_get_table(&self, tag: TableTag) -> io::Result<TableData> {
        self.tables[tag].ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Table entry for tag '{:?}' not found", tag),
            )
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TableData {
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, enum_map::Enum)]
pub enum TableTag {
    Head,
    Hhea,
    Hmtx,
    Maxp,
    Name,
    Cmap,
    Glyf,
    Loca,
    Hdmx,
    Os2,
    Post,
    Cvt,
    Fpgm,
    Prep,
    GlyphDirectory,
    Kerning,
    HorizontalHeader,
    TablesNum,
}

impl TableTag {
    pub fn from_bytes(bytes: [char; 4]) -> io::Result<Self> {
        let tag_str: String = bytes.iter().collect();
        match tag_str.as_str() {
            "cmap" => Ok(TableTag::Cmap),
            "head" => Ok(TableTag::Head),
            "hmtx" => Ok(TableTag::Hmtx),
            "maxp" => Ok(TableTag::Maxp),
            "name" => Ok(TableTag::Name),
            "post" => Ok(TableTag::Post),
            "loca" => Ok(TableTag::Loca),
            "glyf" => Ok(TableTag::Glyf),
            "hdmx" => Ok(TableTag::Hdmx),
            "OS/2" => Ok(TableTag::Os2),
            "cvt " => Ok(TableTag::Cvt),
            "fpgm" => Ok(TableTag::Fpgm),
            "prep" => Ok(TableTag::Prep),
            "GDIR" => Ok(TableTag::GlyphDirectory),
            "kern" => Ok(TableTag::Kerning),
            "hhea" => Ok(TableTag::HorizontalHeader),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown table tag: {}", tag_str),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphMapping {
    pub index: usize,
    pub unicode: usize,
}

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
