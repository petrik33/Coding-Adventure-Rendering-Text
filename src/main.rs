mod text;

use text::{
    render::draw_glyph,
    ttf::{parse_all_glyph_locations, parse_font, parse_glyph},
    FontFile,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut font_file = FontFile::load("res/Roboto-Regular.ttf")?;

    let font_data = parse_font(&mut font_file)?;

    let glyph_locations = parse_all_glyph_locations(&mut font_file, &font_data)?;

    font_file.goto(glyph_locations[133])?; // Check out 128 (question mark) to check render sanity

    let glyph = parse_glyph(&mut font_file)?;
    println!("{:?}", glyph);
    draw_glyph(&glyph)?;

    Ok(())
}
