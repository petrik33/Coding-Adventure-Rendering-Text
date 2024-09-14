use std::{fs::File, io::Write};

use tiny_skia::{Paint, PathBuilder, Pixmap, Point, Stroke, Transform};

use super::{font_data::GlyphData, GlyphPoint};

pub struct Renderer {
    pub offset: Point,
    pub pixmap: Pixmap,
    pub stroke: Stroke,
}

impl Renderer {
    pub fn new(offset_x: f32, offset_y: f32, width: u32, height: u32) -> Self {
        Renderer {
            offset: Point {
                x: offset_x,
                y: offset_y,
            },
            pixmap: Pixmap::new(width, height).unwrap(),
            stroke: Stroke {
                width: 4.0,
                ..Default::default()
            },
        }
    }

    pub fn draw_glyph(&mut self, glyph_data: &GlyphData) -> Result<(), Box<dyn std::error::Error>> {
        let mut contours = Vec::with_capacity(glyph_data.contour_end_indices.len());

        let paint = Paint {
            anti_alias: true,
            ..Default::default()
        };

        let mut prev_index = 0;

        for &end_index in &glyph_data.contour_end_indices {
            let mut pb = PathBuilder::new();
            let start_index = prev_index;

            let first_point = self.convert_glyph_point(glyph_data.points[start_index]);
            pb.move_to(first_point.x, first_point.y);

            let mut current_point_idx = start_index + 2;

            while current_point_idx <= end_index {
                let control_point =
                    self.convert_glyph_point(glyph_data.points[current_point_idx - 1]);
                let point = self.convert_glyph_point(glyph_data.points[current_point_idx]);
                pb.quad_to(control_point.x, control_point.y, point.x, point.y);
                current_point_idx += 2;
            }

            pb.close();

            if let Some(contour) = pb.finish() {
                contours.push(contour);
            }

            prev_index = end_index + 1;
        }

        for contour in contours.iter() {
            self.pixmap
                .stroke_path(contour, &paint, &self.stroke, Transform::identity(), None);
        }

        // Save the pixmap to a PNG file
        let mut file = File::create("glyph.png")?;
        file.write_all(self.pixmap.encode_png()?.as_ref())?;

        Ok(())
    }

    pub fn convert_glyph_point(&self, point: GlyphPoint) -> Point {
        Point {
            x: point.x as f32 + self.offset.x,
            y: (self.pixmap.height() as i64 - point.y) as f32 - self.offset.y,
        }
    }
}
