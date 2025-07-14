use std::fs::File;

use strum::{EnumIter,IntoEnumIterator};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, EnumIter)]
pub enum System {
    NDS,
    GBA,
    GBC,
    GB,
    WonderSwan
}

pub fn get_system_file_extension(system: System) -> Vec<String> {
    match system {
        System::NDS => vec!["nds".to_string()],
        System::GBA => vec!["gba".to_string()],
        System::GBC => vec!["gbc".to_string()],
        System::GB => vec!["gb".to_string()],
        System::WonderSwan => vec!["ws".to_string(), "wsc".to_string()],
    }
}

pub fn get_system_ra_name(system: System) -> &'static str {
    match system {
        System::NDS => "Nintendo DS",
        System::GBA => "Game Boy Advance",
        System::GB => "Game Boy",
        System::GBC => "Game Boy Color",
        System::WonderSwan => "WonderSwan",
    }
}

pub fn determine_rom_system(file_path: &str) -> Option<System> {
    let path = std::path::Path::new(file_path);

    if !path.exists() || !path.is_file() {
        return None;
    }

    let extension = path.extension()?.to_str()?.to_lowercase();

    // Check extension first
    for system in System::iter() {
        let extensions = get_system_file_extension(system);

        if extensions.contains(&extension) {
            return Some(system);
        }
    }

    // If it's a zip, check the contents
    if extension == "zip" {
        let mut zip_file_archive = match zip::ZipArchive::new(File::open(file_path).ok()?) {
            Ok(archive) => archive,
            Err(_) => return None,
        };

        for i in 0..zip_file_archive.len() {
            let file = zip_file_archive.by_index(i).ok()?;
            if let Some(extension) = file.name().rsplit('.').next() {
                for system in System::iter() {
                    let extensions = get_system_file_extension(system);

                    if extensions.contains(&extension.to_lowercase()) {
                        return Some(system);
                    }
                }
            }
        }
    }

    None
}