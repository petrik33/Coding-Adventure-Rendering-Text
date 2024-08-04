use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

const TTF_FILE_SCALER_TYPE_OFFSET: u64 = 4;

pub fn parse_font(font_path: &str) -> io::Result<u16> {
    let mut file = File::open(font_path)?;
    file.seek(SeekFrom::Start(TTF_FILE_SCALER_TYPE_OFFSET))?;
    let mut buffer = [0u8; 2];
    file.read_exact(&mut buffer)?;
    let tables_num = u16::from_le_bytes(buffer);
    Ok(tables_num)
}
