use std::io;

use crate::text::{ttf::GlyphFlags, FontFile, GlyphData, GlyphPoint};

pub fn parse_glyph(file: &mut FontFile) -> io::Result<GlyphData> {
    let i_contours_num = file.read_i16()?;
    if i_contours_num < 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Compound glyps are not supported yet",
        ));
    }

    let contours_num = i_contours_num as usize;
    let mut contour_end_indices = Vec::with_capacity(contours_num);

    let _x_min = file.read_i16()?;
    let _y_min = file.read_i16()?;
    let _x_max = file.read_i16()?;
    let _y_max = file.read_i16()?;

    for _ in 0..contours_num {
        contour_end_indices.push(file.read_u16()? as usize);
    }

    let instructions_size = file.read_u16()?;

    let mut _instructions = Vec::with_capacity(instructions_size as usize);
    for _ in 0..instructions_size {
        _instructions.push(file.read_i8()?);
    }

    let num_points = contour_end_indices
        .last()
        .map_or(1, |&last| last as usize + 1);

    let mut all_masks = Vec::with_capacity(num_points);
    let mut points_saved = 0;

    while points_saved < num_points {
        let glyph_mask = GlyphFlags::from_bits_retain(file.read_bitmask()?);
        all_masks.push(glyph_mask);

        points_saved += 1;

        if glyph_mask.contains(GlyphFlags::Repeat) {
            let repeat_times = file.read_u8()? as usize;
            for _ in 0..repeat_times {
                all_masks.push(glyph_mask);
                points_saved += 1;
            }
        }
    }

    let x_coordinates = parse_glyph_x(file, &all_masks)?;
    let y_coordinates = parse_glyph_y(file, &all_masks)?;

    Ok(GlyphData {
        contour_end_indices,
        points: x_coordinates
            .into_iter()
            .zip(y_coordinates.into_iter())
            .map(|(x, y)| GlyphPoint { x, y })
            .collect(),
    })
}

pub fn parse_glyph_x(file: &mut FontFile, all_masks: &[GlyphFlags]) -> io::Result<Vec<i64>> {
    let mut previous = 0;

    let x_coordinates = all_masks
        .iter()
        .map(|mask| {
            let mut x = previous;

            if mask.contains(GlyphFlags::ShortVectorX) {
                let sign_x = if mask.contains(GlyphFlags::IsPositiveX) {
                    1
                } else {
                    -1
                };
                let offset_x = file.read_u8().unwrap() as i64 * sign_x;
                x += offset_x;
            } else if !mask.contains(GlyphFlags::IsSameX) {
                let offset_x = file.read_i16().unwrap() as i64;
                x += offset_x;
            }

            previous = x;
            x
        })
        .collect();

    Ok(x_coordinates)
}

pub fn parse_glyph_y(file: &mut FontFile, all_masks: &[GlyphFlags]) -> io::Result<Vec<i64>> {
    let mut previous = 0;

    let y_coordinates = all_masks
        .iter()
        .map(|mask| {
            let mut y = previous;

            if mask.contains(GlyphFlags::ShortVectorY) {
                let sign_y = if mask.contains(GlyphFlags::IsPositiveY) {
                    1
                } else {
                    -1
                };
                let offset_y = file.read_u8().unwrap() as i64 * sign_y;
                y += offset_y;
            } else if !mask.contains(GlyphFlags::IsSameY) {
                let offset_y = file.read_i16().unwrap() as i64;
                y += offset_y;
            }

            previous = y;
            y
        })
        .collect();

    Ok(y_coordinates)
}
