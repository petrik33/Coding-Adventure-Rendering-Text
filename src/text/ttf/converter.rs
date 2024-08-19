use crate::text::GlyphPoint;

pub fn convert_point(point: GlyphPoint) -> (f32, f32) {
    (point.x as f32 / 1024.0, point.y as f32 / 1024.0)
}
