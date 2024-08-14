mod math_utils;
mod text_render;

use std::io;
use text_render::ttf::{parse_font, FontFile};

fn main() -> io::Result<()> {
    let path = "res/FiraCode-Regular.ttf";
    let mut file = FontFile::load(path)?;
    let data = parse_font(&mut file)?;
    println!("{:?}", data);
    Ok(())
}
