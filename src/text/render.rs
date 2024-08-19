use std::{fs::File, io::Write};

use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke, Transform};

use super::{ttf::convert_point, GlyphData};

pub fn draw_glyph(glyph_data: &GlyphData) -> Result<(), Box<dyn std::error::Error>> {
    let width = 2048;
    let height = 2048;

    let mut pixmap = Pixmap::new(width, height).unwrap();
    let mut contours = Vec::with_capacity(glyph_data.contour_end_indices.len());

    let paint = Paint {
        anti_alias: true,
        ..Default::default()
    };

    let stroke = Stroke {
        width: 1.0,
        ..Default::default()
    };

    let mut prev_index = 0;

    for &end_index in &glyph_data.contour_end_indices {
        let mut pb = PathBuilder::new();
        let start_index = prev_index;

        // Move to the first point of the contour
        let first_point = &glyph_data.points[start_index];
        pb.move_to(first_point.x as f32, first_point.y as f32);

        // Draw lines to the subsequent points in the contour
        for i in start_index + 1..=end_index {
            let point = &glyph_data.points[i];
            pb.line_to(point.x as f32, point.y as f32);
        }

        pb.close();

        if let Some(contour) = pb.finish() {
            contours.push(contour);
        }

        prev_index = end_index + 1;
    }

    for contour in contours.iter() {
        pixmap.stroke_path(&contour, &paint, &stroke, Transform::identity(), None);
    }

    // Save the pixmap to a PNG file
    let mut file = File::create("glyph.png")?;
    file.write_all(pixmap.encode_png()?.as_ref())?;

    Ok(())
}
