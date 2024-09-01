pub use glyph_flags::GlyphFlags;
pub use parser::{parse_cmap, parse_font, parse_glyph, parse_glyph_offsets};

mod glyph_flags;
mod parser;
mod platform_id;
