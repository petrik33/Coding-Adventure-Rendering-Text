use std::{fs::File, io::Write};

use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke, Transform};

use super::font_data::GlyphData;

const EM_SIZE: u32 = 2048;

pub fn draw_glyph(glyph_data: &GlyphData) -> Result<(), Box<dyn std::error::Error>> {
    let width = EM_SIZE;
    let height = EM_SIZE;
    let offset_x = EM_SIZE as f32 / 32f32;
    let offset_y = EM_SIZE as f32 / 32f32;

    let mut pixmap = Pixmap::new(width, height).unwrap();
    let mut contours = Vec::with_capacity(glyph_data.contour_end_indices.len());

    let paint = Paint {
        anti_alias: true,
        ..Default::default()
    };

    let stroke = Stroke {
        width: 4.0,
        ..Default::default()
    };

    let mut prev_index = 0;

    for &end_index in &glyph_data.contour_end_indices {
        let mut pb = PathBuilder::new();
        let start_index = prev_index;

        let first_point = &glyph_data.points[start_index];
        pb.move_to(first_point.x as f32 + offset_x, (height as i64 - first_point.y) as f32 - offset_y);

        for i in start_index + 1..=end_index {
            let point = &glyph_data.points[i];
            pb.line_to(point.x as f32 + offset_x, (height as i64 - point.y) as f32 - offset_y);
        }

        // let mut current_point_idx = start_index + 2;

        // while current_point_idx <= end_index {
        //     let control_point = &glyph_data.points[current_point_idx - 1];
        //     let point = &glyph_data.points[current_point_idx];
        //     pb.quad_to(
        //         control_point.x as f32,
        //         control_point.y as f32,
        //         point.x as f32,
        //         point.y as f32,
        //     );
        //     current_point_idx += 2;
        // }

        pb.close();

        if let Some(contour) = pb.finish() {
            contours.push(contour);
        }

        prev_index = end_index + 1;
    }

    for contour in contours.iter() {
        pixmap.stroke_path(contour, &paint, &stroke, Transform::identity(), None);
    }

    // Save the pixmap to a PNG file
    let mut file = File::create("glyph.png")?;
    file.write_all(pixmap.encode_png()?.as_ref())?;

    Ok(())
}
