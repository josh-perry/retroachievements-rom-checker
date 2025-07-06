// https://docs.retroachievements.org/developer-docs/game-identification.html

use std::io::{Read, Seek, SeekFrom};
use md5;

pub fn calculate_nds_file_hash(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // A NDS ROM has a 0x160 byte header. In this header are pointers to icon/title information and to the boot code for both processors.
    // The hash method combines the header, the two pieces of boot code, and the icon/title information and hashes the result.
    // The icon/title information is 0xA00 bytes starting at the address stored in the header at $68
    // The arm9 code address is stored at $20 in the header, and the size is stored at $2C in the header
    // The arm7 code address is stored at $30 in the header, and the size is stored at $3C in the header

    let mut file = std::fs::File::open(file_path)?;

    let mut header = [0; 0x160];
    file.read_exact(&mut header)?;

    let icon_title_offset = u32::from_le_bytes(header[0x68..0x6C].try_into()?) as u64;
    let icon_title_size = 0xA00;

    let arm9_offset = u32::from_le_bytes(header[0x20..0x24].try_into()?) as u64;
    let arm9_size = u32::from_le_bytes(header[0x2C..0x30].try_into()?) as usize;

    let arm7_offset = u32::from_le_bytes(header[0x30..0x34].try_into()?) as u64;
    let arm7_size = u32::from_le_bytes(header[0x3C..0x40].try_into()?) as usize;

    let icon_title_data = read_file_section(&mut file, icon_title_offset, icon_title_size)?;
    let arm9_data = read_file_section(&mut file, arm9_offset, arm9_size)?;
    let arm7_data = read_file_section(&mut file, arm7_offset, arm7_size)?;

    let mut hasher = md5::Context::new();
    hasher.consume(&header);
    hasher.consume(&arm9_data);
    hasher.consume(&arm7_data);
    hasher.consume(&icon_title_data);

    Ok(format!("{:x}", hasher.finalize()))
}

fn read_file_section(file: &mut std::fs::File, offset: u64, size: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = vec![0; size];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}