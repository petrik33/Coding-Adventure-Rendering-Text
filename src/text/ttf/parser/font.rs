use std::io;

use enum_map::EnumMap;

use crate::text::{font_data::TableTag, FontData, FontFile, TableData};

pub fn parse_font(file: &mut FontFile) -> io::Result<FontData> {
    let _scaler_type = file.read_u32()?;
    let num_tables = file.read_u16()?;
    let _search_range = file.read_u16()?;
    let _entry_selector = file.read_u16()?;
    let _range_shift = file.read_u16()?;

    let mut tables: EnumMap<TableTag, Option<TableData>> = EnumMap::default();

    for _ in 0..num_tables {
        match TableTag::from_bytes(file.read_tag()?) {
            Ok(tag) => {
                let table = TableData {
                    checksum: file.read_u32()?,
                    offset: file.read_u32()?,
                    length: file.read_u32()?,
                };
                tables[tag] = Some(table);
            }
            Err(e) => {
                println!("Warning: {}", e);
                file.skip_bytes(12)?;
            }
        }
    }

    Ok(FontData { tables })
}
