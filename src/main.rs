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

    let rendered_char_idx = 66;
    let char_mapping = cmap[rendered_char_idx];
    println!(
        "Printing character with index {}, mapped to {}, with unicode value {}",
        rendered_char_idx, char_mapping.index, char_mapping.unicode
    );

    font_file.goto(glyph_locations[char_mapping.unicode])?;

    let glyph = parse_glyph(&mut font_file)?;
    println!("{:?}", glyph);
    draw_glyph(&glyph)?;

    Ok(())
}
