mod text;

use text::{
    render::draw_glyph,
    ttf::{parse_font, parse_glyph},
    FontFile,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut font_file = FontFile::load("res/Roboto-Regular.ttf")?;
    let font_data = parse_font(&mut font_file)?;
    println!("{:?}", font_data);
    font_file.goto(font_data.tables["glyf"].offset)?;
    let glyph0 = parse_glyph(&mut font_file)?;
    println!("{:?}", glyph0);
    draw_glyph(&glyph0)?;

    Ok(())
}
