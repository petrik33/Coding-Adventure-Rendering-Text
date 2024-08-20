use crate::text::{FontData, FontFile};
use std::io;

pub fn parse_all_glyph_locations(
    file: &mut FontFile,
    font_data: &FontData,
) -> io::Result<Vec<u32>> {
    file.goto(font_data.tables["maxp"].offset + 4)?; // Skip unused version
    let num_glyphs = file.read_u16()?;

    file.goto(font_data.tables["head"].offset)?;
    file.skip_bytes(50)?; // Skip a few unused fields TODO: change with unused variable declarations

    let is_two_byte_entry = file.read_i16()? == 0;
    let entry_offset = if is_two_byte_entry { 2u32 } else { 4u32 };

    let location_table_start = font_data.tables["loca"].offset;
    let glyph_table_start = font_data.tables["glyf"].offset;

    let mut glyph_locations = Vec::with_capacity(num_glyphs as usize);

    for i in 0..num_glyphs {
        file.goto(location_table_start + i as u32 * entry_offset)?;
        let glyph_data_offset = if is_two_byte_entry {
            file.read_u16()? as u32 * 2u32
        } else {
            file.read_u32()?
        };
        glyph_locations.push(glyph_table_start + glyph_data_offset);
    }

    Ok(glyph_locations)
}
