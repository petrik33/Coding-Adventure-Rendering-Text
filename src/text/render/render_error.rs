#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RenderError {
    FontNotLoaded,
    GlyphNotFound,
    OutlineError,
}

impl core::fmt::Display for RenderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FontNotLoaded => write!(f, "font not loaded"),
            Self::GlyphNotFound => write!(f, "glyph not found"),
            Self::OutlineError => write!(f, "could not outline glyph"),
        }
    }
}

impl std::error::Error for RenderError {}
