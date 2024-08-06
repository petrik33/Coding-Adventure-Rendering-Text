use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

pub fn parse_ttf_font(file: &mut TtfFontFile) -> io::Result<TtfFontData> {
    file.skip_bits(u32::BITS)?; // `scaler type`
    let num_tables = file.read_u16()?;
    file.skip_bits(u16::BITS * 3)?; // `search range`, `entry selector`, `range shift`

    let mut tables = Vec::new();
    tables.reserve(num_tables as usize);

    for _ in 0..num_tables {
        tables.push(TtfGlyphTable {
            tag: file.read_tag()?,
            checksum: file.read_u32()?,
            offset: file.read_u32()?,
            length: file.read_u32()?,
        });
    }

    Ok(TtfFontData { num_tables, tables })
}

pub struct TtfFontFile {
    file: File,
}

#[derive(Debug)]
pub struct TtfFontData {
    num_tables: u16,
    tables: Vec<TtfGlyphTable>,
}

#[derive(Debug)]
pub struct TtfGlyphTable {
    tag: [char; 4],
    checksum: u32,
    offset: u32,
    length: u32,
}

impl TtfFontFile {
    pub fn load(font_path: &str) -> io::Result<Self> {
        File::open(font_path).map(|file| TtfFontFile { file })
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

    pub fn read_tag(&mut self) -> io::Result<[char; 4]> {
        let mut buffer = [0u8; 4];
        self.file.read_exact(&mut buffer)?;
        let tag: [char; 4] = [
            buffer[0] as char,
            buffer[1] as char,
            buffer[2] as char,
            buffer[3] as char,
        ];
        Ok(tag)
    }

    pub fn skip_bits(&mut self, num_bits: u32) -> io::Result<()> {
        self.file
            .seek(SeekFrom::Current((num_bits / 8) as i64))
            .map(|_| ())
    }
}
