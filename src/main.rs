mod text_render;

use std::io;
use text_render::{parse_ttf_font, TtfFontFile};

fn main() -> io::Result<()> {
    let path = "res/FiraCode-Regular.ttf";
    let mut file = TtfFontFile::load(path)?;
    let data = parse_ttf_font(&mut file)?;
    println!("{:?}", data);
    Ok(())
}
