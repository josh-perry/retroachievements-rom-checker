// https://docs.retroachievements.org/developer-docs/game-identification.html

use std::{fs::{File}, io::{Read, Seek, SeekFrom}};
use md5;

use crate::system;

trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

pub fn get_hash_function(system: &system::System) -> Option<fn(&str) -> Result<String, Box<dyn std::error::Error>>> {
    match system {
        system::System::NDS => Some(calculate_nds_file_hash),
        system::System::GBA => Some(calculate_whole_file_hash),
        system::System::GBC => Some(calculate_whole_file_hash),
        system::System::GB => Some(calculate_whole_file_hash),
        system::System::WonderSwan => Some(calculate_whole_file_hash),
    }
}

fn calculate_whole_file_hash(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if file_path.ends_with(".zip") {
        let mut zip_file_archive = zip::ZipArchive::new(File::open(file_path)?)?;
        if zip_file_archive.len() == 0 {
            return Err("ZIP archive is empty".into());
        }

        let mut first_file = zip_file_archive.by_index(0)?;
        let mut buffer = Vec::new();
        first_file.read_to_end(&mut buffer)?;
        let mut hasher = md5::Context::new();
        hasher.consume(&buffer);
        return Ok(format!("{:x}", hasher.finalize()));
    }

    let mut file = File::open(file_path)?;
    let mut hasher = md5::Context::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.consume(&buffer);
    Ok(format!("{:x}", hasher.finalize()))
}

fn calculate_nds_file_hash(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // A NDS ROM has a 0x160 byte header. In this header are pointers to icon/title information and to the boot code for both processors.
    // The hash method combines the header, the two pieces of boot code, and the icon/title information and hashes the result.
    // The icon/title information is 0xA00 bytes starting at the address stored in the header at $68
    // The arm9 code address is stored at $20 in the header, and the size is stored at $2C in the header
    // The arm7 code address is stored at $30 in the header, and the size is stored at $3C in the header

    if !file_path.ends_with(".nds") && !file_path.ends_with(".zip") {
        return Err("File is not a valid NDS ROM or ZIP archive".into());
    }

    fn get_nds_file(file_path: &str) -> Result<Box<dyn ReadSeek>, Box<dyn std::error::Error>> {
        if file_path.ends_with(".zip") {
            let mut zip_file_archive = zip::ZipArchive::new(File::open(file_path)?)?;

            let file_names: Vec<String> = zip_file_archive.file_names().map(|name| name.to_string()).collect();
            for file_name in file_names {
                if file_name.ends_with(".nds") {
                    // TODO: figure out how to do this without copying the file contents into a buffer
                    // This is necessary (???) because the zip_file_archive dies before we read the contents down below.
                    let mut file = zip_file_archive.by_name(&file_name)?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    return Ok(Box::new(std::io::Cursor::new(buffer)));
                }
            }

            return Err("No valid NDS file found in the ZIP archive".into());
        } else {
            return Ok(Box::new(File::open(file_path)?));
        }
    }

    let mut nds_file = get_nds_file(file_path)?;
    
    if nds_file.seek(SeekFrom::Start(0)).is_err() {
        print!("Failed to seek to the start of the file: {}", file_path);
        return Err("Failed to seek to the start of the file".into());
    }

    let mut header = [0; 0x160];
    nds_file.read_exact(&mut header)?;

    let icon_title_offset = u32::from_le_bytes(header[0x68..0x6C].try_into()?) as u64;
    let icon_title_size = 0xA00;

    let arm9_offset = u32::from_le_bytes(header[0x20..0x24].try_into()?) as u64;
    let arm9_size = u32::from_le_bytes(header[0x2C..0x30].try_into()?) as usize;

    let arm7_offset = u32::from_le_bytes(header[0x30..0x34].try_into()?) as u64;
    let arm7_size = u32::from_le_bytes(header[0x3C..0x40].try_into()?) as usize;

    let icon_title_data = read_file_section(&mut nds_file, icon_title_offset, icon_title_size)?;
    let arm9_data = read_file_section(&mut nds_file, arm9_offset, arm9_size)?;
    let arm7_data = read_file_section(&mut nds_file, arm7_offset, arm7_size)?;

    let mut hasher = md5::Context::new();
    hasher.consume(&header);
    hasher.consume(&arm9_data);
    hasher.consume(&arm7_data);
    hasher.consume(&icon_title_data);

    Ok(format!("{:x}", hasher.finalize()))
}

fn read_file_section(file: &mut impl ReadSeek, offset: u64, size: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = vec![0; size];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}