mod text_render;

fn main() {
    let font_path = "res/FiraCode-Regular.ttf";
    match text_render::parse_font(font_path) {
        Ok(tables_num) => println!("The u16 number after skipping 4 bytes is: {}", tables_num),
        Err(e) => eprintln!("Error parsing font: {}", e),
    }
}
