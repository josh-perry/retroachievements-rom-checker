use strum::{EnumIter};

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