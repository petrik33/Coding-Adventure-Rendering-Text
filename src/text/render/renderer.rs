use crate::text::render::{OutlineBuilder, RenderError};
use std::{fs::File, io::Write};

type BoxedError = Box<dyn std::error::Error>;

pub struct Renderer<'a> {
    pub offset: tiny_skia::Point,
    pub pixmap: tiny_skia::Pixmap,
    pub stroke: tiny_skia::Stroke,
    pub font_face: Option<ttf_parser::Face<'a>>,
}

impl<'a> Renderer<'a> {
    pub fn new(offset_x: f32, offset_y: f32, width: u32, height: u32) -> Self {
        Renderer {
            offset: tiny_skia::Point {
                x: offset_x,
                y: offset_y,
            },
            pixmap: tiny_skia::Pixmap::new(width, height).unwrap(),
            stroke: tiny_skia::Stroke {
                width: 4.0,
                ..Default::default()
            },
            font_face: None,
        }
    }

    pub fn load_font(&mut self, data: &'a [u8]) -> Result<(), BoxedError> {
        let parsed_face = ttf_parser::Face::parse(data, 0)?;
        self.font_face = Some(parsed_face);
        Ok(())
    }

    pub fn draw_glyph(&mut self, glyph_code: char) -> Result<(), BoxedError> {
        let font_face = self.font_face.as_ref().ok_or(RenderError::FontNotLoaded)?;
        let glyph_id = font_face
            .glyph_index(glyph_code)
            .ok_or(RenderError::GlyphNotFound)?;

        let mut builder = OutlineBuilder::new();
        let _bbox = font_face
            .outline_glyph(glyph_id, &mut builder)
            .ok_or(RenderError::OutlineError)?;

        // let transform = Transform::from_rotate(180f32);

        let paint = tiny_skia::Paint {
            anti_alias: true,
            ..Default::default()
        };

        let invert_y_axis_transform = tiny_skia::Transform::from_row(
            1.0,
            0.0,
            0.0,
            -1.0,
            self.offset.x,
            self.pixmap.height() as f32 - self.offset.y,
        );

        for contour in &builder.contours {
            self.pixmap
                .stroke_path(contour, &paint, &self.stroke, invert_y_axis_transform, None);
        }

        Ok(())
    }

    pub fn save(&self, filename: &str) -> Result<(), BoxedError> {
        let mut file = File::create(filename)?;
        file.write_all(self.pixmap.encode_png()?.as_ref())?;
        Ok(())
    }
}
