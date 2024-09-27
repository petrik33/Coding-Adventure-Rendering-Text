mod text;

use std::fs;
use text::render::Renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read("res/Roboto-Regular.ttf")?;
    let mut renderer = Renderer::new(128.0, 128.0, 2048, 2048);
    renderer.load_font(&data)?;
    renderer.draw_glyph('G')?;
    renderer.save("glyph.png")?;
    Ok(())
}
