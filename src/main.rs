mod text;

use text::{
    render::draw_glyph,
    ttf::{parse_cmap, parse_font, parse_glyph, parse_glyph_offsets},
    FontFile,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut font_file = FontFile::load("res/Roboto-Regular.ttf")?;

    let font_data = parse_font(&mut font_file)?;
    let glyph_locations = parse_glyph_offsets(&mut font_file, &font_data)?;
    let cmap = parse_cmap(&mut font_file, &font_data)?;

    println!("{:?}", cmap);

    let rendered_char_idx: usize = 66;
    let char_mapping = cmap[cmap
        .binary_search_by(|mapping| mapping.unicode.cmp(&rendered_char_idx))
        .map_err(|_| {
            format!(
                "Character mapping not found for index {}",
                rendered_char_idx
            )
        })?];

    println!(
        "Printing character with index {}, mapped to {}, with unicode value {}",
        rendered_char_idx, char_mapping.index, char_mapping.unicode
    );

    font_file.goto(glyph_locations[char_mapping.index])?;

    let glyph = parse_glyph(&mut font_file)?;
    println!("{:?}", glyph);
    draw_glyph(&glyph)?;

    Ok(())
}
