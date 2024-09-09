use std::io;

use cmap_formats::FORMAT_12;

use crate::text::{
    font_data::{GlyphMapping, TableTag},
    ttf::platform_id::{platform_id, unicode_platform_specific_id, windows_platform_specific_id},
    FontData, FontFile,
};

pub fn parse_cmap(file: &mut FontFile, font_data: &FontData) -> io::Result<Vec<GlyphMapping>> {
    let mut cmap = Vec::new();

    let cmap_offset = font_data.try_get_table(TableTag::Cmap)?.offset;
    file.goto(cmap_offset)?;

    let _version = file.read_u16()?;
    let num_subtables = file.read_u16()? as usize;

    let mut subtable_offset = 0;
    let mut selected_unicode_version_id = 0;
    let mut unicode_subtable_selected = false;
    let mut subtable_selected = false;

    for _ in 0..num_subtables {
        let platform_id = file.read_u16()?;
        let platform_specific_id = file.read_u16()?;
        let offset = file.read_u32()?;

        match platform_id {
            platform_id::UNICODE
                if !unicode_subtable_selected
                    || platform_specific_id >= selected_unicode_version_id =>
            {
                if is_supported_unicode_platform_specific_id(platform_specific_id) {
                    subtable_offset = offset;
                    selected_unicode_version_id = platform_specific_id;
                    unicode_subtable_selected = true;
                    subtable_selected = true;
                }
            }
            platform_id::MICROSOFT if !unicode_subtable_selected => {
                if is_supported_windows_platform_specific_id(platform_specific_id) {
                    subtable_offset = offset;
                    subtable_selected = true;
                }
            }
            _ => {}
        }
    }

    if !subtable_selected {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "None of the cmap subtables is supported",
        ));
    }

    file.goto(cmap_offset + subtable_offset)?;
    let format = file.read_u16()?;

    if !is_supported_cmap_format(format) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Cmap subtable format {} is not supported", format),
        ));
    }

    let mut has_read_missing_char_glyph = false;

    match format {
        cmap_formats::FORMAT_4 => {
            let _length = file.read_u16()?;
            let _language_code = file.read_u16()?;
            let seg_count_x2 = file.read_u16()?;
            let seg_count = (seg_count_x2 / 2) as usize;
            file.skip_bytes(6)?; // Skip: searchRange, entrySelector, rangeShif?t

            let mut end_codes = vec![0; seg_count];
            for end_code in &mut end_codes {
                *end_code = file.read_u16()?;
            }

            file.skip_bytes(2)?; // Reserved pad

            let mut start_codes = vec![0u32; seg_count];
            for start_code in &mut start_codes {
                *start_code = file.read_u16()? as u32;
            }

            let mut id_deltas = vec![0u32; seg_count];
            for id_delta in &mut id_deltas {
                *id_delta = file.read_u16()? as u32;
            }

            let mut read_loc = file.get_location()?;
            let mut id_range_offsets = vec![(0, 0); seg_count];
            for id_range_offset in &mut id_range_offsets {
                let offset = file.read_u16()?;
                *id_range_offset = (offset, read_loc);
                read_loc += 2;
            }

            for i in 0..start_codes.len() {
                let end_code = end_codes[i];
                let mut curr_code = start_codes[i];

                // not sure about this (hack to avoid out of bounds on a specific font)
                if curr_code == u16::MAX as u32 {
                    break;
                }

                while curr_code <= end_code as u32 {
                    let mut glyph_index: u32;
                    if id_range_offsets[i].0 == 0 {
                        glyph_index = (curr_code + id_deltas[i]) % u16::MAX as u32;
                    } else {
                        let reader_location_old = file.get_location()?;
                        let range_offset_location =
                            id_range_offsets[i].1 + id_range_offsets[i].0 as u64;
                        let glyph_index_array_location =
                            2 * (curr_code - start_codes[i]) as u64 + range_offset_location;

                        file.goto(glyph_index_array_location as u32)?;
                        glyph_index = file.read_u16()? as u32;

                        if glyph_index != 0 {
                            glyph_index = (glyph_index + id_deltas[i] as u32) % u16::MAX as u32;
                        }

                        file.goto(reader_location_old as u32)?;
                    }

                    cmap.push(GlyphMapping {
                        index: glyph_index as usize - 1,
                        unicode: curr_code as usize,
                    });

                    has_read_missing_char_glyph |= glyph_index == 0;
                    curr_code += 1;
                }
            }
        }
        FORMAT_12 => {
            file.skip_bytes(10)?; // Skip: reserved, subtableByteLengthInlcudingHeader, languageCode
            let num_groups = file.read_u32()?;

            for _ in 0..num_groups {
                let start_char_code = file.read_u32()?;
                let end_char_code = file.read_u32()?;
                let start_glyph_index = file.read_u32()?;

                let num_chars = end_char_code - start_char_code + 1;
                for char_code_offset in 0..num_chars {
                    let glyph_index = (start_char_code + char_code_offset) as usize;

                    cmap.push(GlyphMapping {
                        index: glyph_index as usize - 1,
                        unicode: (start_glyph_index + char_code_offset) as usize,
                    });

                    has_read_missing_char_glyph |= glyph_index == 0;
                }
            }
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported cmap subtable format",
            ));
        }
    }

    if !has_read_missing_char_glyph {
        cmap.push(GlyphMapping {
            index: 0,
            unicode: u16::MAX as usize,
        });
    }

    Ok(cmap)
}

const fn is_supported_unicode_platform_specific_id(id: u16) -> bool {
    matches!(
        id,
        unicode_platform_specific_id::VERSION_1_0
            | unicode_platform_specific_id::VERSION_1_1
            | unicode_platform_specific_id::UNICODE_2_0_OR_LATER_BMP_ONLY
            | unicode_platform_specific_id::UNICODE_2_0_OR_LATER_NON_BMP_ALLOWED
    )
}

const fn is_supported_windows_platform_specific_id(id: u16) -> bool {
    matches!(
        id,
        windows_platform_specific_id::UNICODE_BMP_ONLY..windows_platform_specific_id::UNICODE_UCS_4
    )
}

const fn is_supported_cmap_format(format: u16) -> bool {
    matches!(format, cmap_formats::FORMAT_4 | cmap_formats::FORMAT_14)
}

pub mod cmap_formats {
    /// Format 0: Macintosh standard character to glyph mapping.
    pub const FORMAT_0: u16 = 0;

    /// Format 2: Mixed 8/16-bit mapping useful for Japanese, Chinese, and Korean.
    pub const FORMAT_2: u16 = 2;

    /// Format 4: 16-bit mappings.
    pub const FORMAT_4: u16 = 4;

    /// Format 6: Dense 16-bit mappings.
    pub const FORMAT_6: u16 = 6;

    /// Format 8: Mixed 16/32-bit mapping.
    pub const FORMAT_8: u16 = 8;

    /// Format 10: Mixed 16/32-bit mapping.
    pub const FORMAT_10: u16 = 10;

    /// Format 12: Pure 32-bit mappings.
    pub const FORMAT_12: u16 = 12;

    /// Format 13: Specialized use, structurally the same as format 12 but with a different interpretation of the data.
    pub const FORMAT_13: u16 = 13;

    /// Format 14: Specialized use for Unicode variation selectors.
    pub const FORMAT_14: u16 = 14;
}
