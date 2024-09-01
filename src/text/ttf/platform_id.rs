pub mod platform_id {
    pub const UNICODE: u16 = 0;
    /// The use of the Macintosh platformID is currently discouraged.
    /// Subtables with a Macintosh platformID are only required for backwards compatibility with QuickDraw
    /// and will be synthesized from Unicode-based subtables if ever needed.
    pub const MACINTOSH: u16 = 1;
    /// Reserved; Do not use
    pub const RESERVED: u16 = 2;
    pub const MICROSOFT: u16 = 3;
}

pub mod unicode_platform_specific_id {
    pub const VERSION_1_0: u16 = 0;
    pub const VERSION_1_1: u16 = 1;
    /// ISO 10646 1993 semantics (deprecated)
    pub const ISO_10646_1993: u16 = 2;
    /// Unicode 2.0 or later semantics (BMP only)
    pub const UNICODE_2_0_OR_LATER_BMP_ONLY: u16 = 3;
    /// Unicode 2.0 or later semantics (non-BMP characters allowed)
    pub const UNICODE_2_0_OR_LATER_NON_BMP_ALLOWED: u16 = 4;
    pub const UNICODE_VARIATION_SEQUENCES: u16 = 5;
    /// The Unicode platform's platform-specific ID 6 was intended to mark a 'cmap' subtable as one used by a last resort font.
    /// This is not required by any Apple platform.
    pub const LAST_RESORT: u16 = 6;
}

/// When the platformID is 1 (Macintosh), the platformSpecificID is a QuickDraw script code.
pub mod macintosh_platform_specific_id {
    pub const ROMAN: u16 = 0;
    pub const JAPANESE: u16 = 1;
    pub const TRADITIONAL_CHINESE: u16 = 2;
    pub const KOREAN: u16 = 3;
    pub const ARABIC: u16 = 4;
    pub const HEBREW: u16 = 5;
    pub const GREEK: u16 = 6;
    pub const RUSSIAN: u16 = 7;
    pub const R_SYMBOL: u16 = 8;
    pub const DEVANAGARI: u16 = 9;
    pub const GURMUKHI: u16 = 10;
    pub const GUJARATI: u16 = 11;
    pub const ORIYA: u16 = 12;
    pub const BENGALI: u16 = 13;
    pub const TAMIL: u16 = 14;
    pub const TELUGU: u16 = 15;
    pub const KANNADA: u16 = 16;
    pub const MALAYALAM: u16 = 17;
    pub const SINHALESE: u16 = 18;
    pub const BURMESE: u16 = 19;
    pub const KHMER: u16 = 20;
    pub const THAI: u16 = 21;
    pub const LAOTIAN: u16 = 22;
    pub const GEORGIAN: u16 = 23;
    pub const ARMENIAN: u16 = 24;
    pub const SIMPLIFIED_CHINESE: u16 = 25;
    pub const TIBETAN: u16 = 26;
    pub const MONGOLIAN: u16 = 27;
    pub const GEEZ: u16 = 28;
    pub const SLAVIC: u16 = 29;
    pub const VIETNAMESE: u16 = 30;
    pub const SINDHI: u16 = 31;
    pub const UNINTERPRETED: u16 = 32;
}

pub mod windows_platform_specific_id {
    pub const SYMBOL: u16 = 0;
    /// UCS-2
    pub const UNICODE_BMP_ONLY: u16 = 1;
    pub const SHIFT_JIS: u16 = 2;
    pub const PRC: u16 = 3;
    pub const BIG_FIVE: u16 = 4;
    pub const JOHAB: u16 = 5;
    pub const UNICODE_UCS_4: u16 = 10;
}
