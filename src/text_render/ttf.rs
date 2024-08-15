use crate::math_utils::flag_is_on;
use bitflags::bitflags;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct FontFile {
    file: File,
}

#[derive(Debug)]
pub struct FontData {
    num_tables: u16,
    tables: Vec<GlyphTable>,
}

#[derive(Debug)]
pub struct GlyphTable {
    tag: Tag,
    checksum: u32,
    offset: u32,
    length: u32,
}

type Tag = [char; 4];

#[derive(Debug)]
pub struct GlyphData {
    contour_end_indices: Vec<u16>,
    points: Vec<GlyphPoint>,
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphPoint {
    x: i64,
    y: i64,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GlyphFlags: u8 {
        ///	If set, the point is on the curve;
        /// Otherwise, it is off the curve.
        const OnCurve = 1;

        /// If set, the corresponding x-coordinate is 1 byte long;
        /// Otherwise, the corresponding x-coordinate is 2 bytes long
        const ShortVectorX = 1 << 1;

        /// If set, the corresponding y-coordinate is 1 byte long;
        /// Otherwise, the corresponding y-coordinate is 2 bytes long
        const ShortVectorY = 1 << 2;

        /// If set, the next byte specifies the number of additional times this set of flags is to be repeated.
        /// In this way, the number of flags listed can be smaller than the number of points in a character.
        const Repeat = 1 << 3;

        /// This flag has one of two meanings, depending on how the x-Short Vector flag is set.
        /// If the x-Short Vector bit is set, this bit describes the sign of the value, with a value of 1 equalling positive and a zero value negative.
        /// If the x-short Vector bit is not set, and this bit is set, then the current x-coordinate is the same as the previous x-coordinate.
        /// If the x-short Vector bit is not set, and this bit is not set, the current x-coordinate is a signed 16-bit delta vector. In this case, the delta vector is the change in x
        const IsSameX = 1 << 4;

        /// This flag has one of two meanings, depending on how the y-Short Vector flag is set.
        /// If the y-Short Vector bit is set, this bit describes the sign of the value, with a value of 1 equalling positive and a zero value negative.
        /// If the y-short Vector bit is not set, and this bit is set, then the current y-coordinate is the same as the previous y-coordinate.
        /// If the y-short Vector bit is not set, and this bit is not set, the current y-coordinate is a signed 16-bit delta vector. In this case, the delta vector is the change in y
        const IsSameY = 1 << 5;

        /// Set to zero
        const Reserved = 1 << 6 | 1 << 7;
    }
}

pub fn parse_font(file: &mut FontFile) -> io::Result<FontData> {
    file.skip_bits(u32::BITS)?; // `scaler type`
    let num_tables = file.read_u16()?;
    file.skip_bits(u16::BITS * 3)?; // `search range`, `entry selector`, `range shift`

    let mut tables = Vec::with_capacity(num_tables as usize);
    let mut tables_dictionary = HashMap::with_capacity(num_tables as usize);

    for _ in 0..num_tables {
        let table = GlyphTable {
            tag: file.read_tag()?,
            checksum: file.read_u32()?,
            offset: file.read_u32()?,
            length: file.read_u32()?,
        };
        tables_dictionary.insert(String::from_iter(table.tag.iter()), table.offset);
        tables.push(table);
    }

    file.goto(tables_dictionary["glyf"])?;
    let glyph0 = parse_glyph(file)?;
    println!("{:?}", glyph0);

    Ok(FontData { num_tables, tables })
}

pub fn parse_glyph(file: &mut FontFile) -> io::Result<GlyphData> {
    let f_contours_num = file.read_i16()?;
    if f_contours_num <= 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Negative number of glyph table indices is not supported",
        ));
    }

    let contours_num = f_contours_num as usize;
    let mut contour_end_indices = vec![0u16; contours_num];

    file.skip_bytes(8)?; // `bounds size`

    for i in 0..contours_num {
        contour_end_indices[i] = file.read_u16()?;
    }

    let num_points = contour_end_indices
        .last()
        .map_or(0, |&last| last as usize + 1);
    let all_masks: Vec<GlyphFlags> = Vec::with_capacity(num_points);

    let instructions_size = file.read_u16()?;
    file.skip_bytes(instructions_size as u32)?;

    let mut all_masks = Vec::with_capacity(num_points);
    let mut points_saved = 0;

    while points_saved < num_points {
        let glyph_mask = GlyphFlags::from_bits_retain(file.read_byte()?);
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

    let mut points = Vec::with_capacity(num_points);
    let mut current_point = GlyphPoint { x: 0, y: 0 };

    for mask in all_masks {
        current_point = parse_glyph_coordinates(file, mask, current_point)?;
        points.push(current_point);
    }

    Ok(GlyphData {
        contour_end_indices,
        points,
    })
}

pub fn parse_glyph_coordinates(
    file: &mut FontFile,
    mask: GlyphFlags,
    previous: GlyphPoint,
) -> io::Result<GlyphPoint> {
    Ok(GlyphPoint {
        x: previous.x
            + if mask.contains(GlyphFlags::ShortVectorX) {
                let sign_x = mask.contains(GlyphFlags::IsSameX) as i64 * 2 - 1;
                file.read_u8()? as i64 * sign_x
            } else {
                file.read_i16()? as i64
            },
        y: previous.y
            + if mask.contains(GlyphFlags::ShortVectorY) {
                let sign_y = mask.contains(GlyphFlags::IsSameY) as i64 * 2 - 1;
                file.read_u8()? as i64 * sign_y
            } else {
                file.read_i16()? as i64
            },
    })
}

impl FontFile {
    pub fn load(font_path: &str) -> io::Result<Self> {
        File::open(font_path).map(|file| FontFile { file })
    }

    pub fn read_i16(&mut self) -> io::Result<i16> {
        let mut buffer = [0u8; 2];
        self.file.read_exact(&mut buffer)?;
        Ok(i16::from_be_bytes(buffer))
    }

    pub fn read_byte(&mut self) -> io::Result<u8> {
        self.read_u8()
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8; 1];
        self.file.read_exact(&mut buffer)?;
        Ok(u8::from_be_bytes(buffer))
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0u8; 2];
        self.file.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0u8; 4];
        self.file.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    pub fn read_tag(&mut self) -> io::Result<Tag> {
        let mut buffer = [0u8; 4];
        self.file.read_exact(&mut buffer)?;
        let tag: Tag = [
            buffer[0] as char,
            buffer[1] as char,
            buffer[2] as char,
            buffer[3] as char,
        ];
        Ok(tag)
    }

    pub fn goto(&mut self, offset: u32) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(offset as u64)).map(|_| ())
    }

    pub fn skip_bits(&mut self, num_bits: u32) -> io::Result<()> {
        self.file
            .seek(SeekFrom::Current((num_bits / 8) as i64))
            .map(|_| ())
    }

    pub fn skip_bytes(&mut self, num_bytes: u32) -> io::Result<()> {
        self.file
            .seek(SeekFrom::Current(num_bytes as i64))
            .map(|_| ())
    }
}
