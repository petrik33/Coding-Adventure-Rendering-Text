use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

pub struct FontFile {
    pub file: File,
}

impl FontFile {
    pub fn load(font_path: &str) -> io::Result<Self> {
        File::open(font_path).map(|file| FontFile { file })
    }

    pub fn read_i8(&mut self) -> io::Result<i8> {
        let mut buffer = [0u8; 1];
        self.file.read_exact(&mut buffer)?;
        Ok(i8::from_be_bytes(buffer))
    }

    pub fn read_i16(&mut self) -> io::Result<i16> {
        let mut buffer = [0u8; 2];
        self.file.read_exact(&mut buffer)?;
        Ok(i16::from_be_bytes(buffer))
    }

    pub fn read_bitmask(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8; 1];
        self.file.read_exact(&mut buffer)?;
        Ok(u8::from_be_bytes(buffer))
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
