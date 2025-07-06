use std::io::{Read, Seek, SeekFrom};
use md5;
use toml;

use rust_fuzzy_search::{fuzzy_search_best_n};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserConfig {
    api_key: String,
    roms_folder: String,
}

fn get_user_config() -> UserConfig {
    let config_filename = "config.toml";

    if !std::path::Path::new(config_filename).exists() {
        panic!("Configuration file '{}' does not exist. Please create it with your API key and ROMs folder.", config_filename);
    }

    let file_contents = std::fs::read_to_string(config_filename)
        .expect("Unable to read configuration file");

    let config: UserConfig = toml::from_str(&file_contents)
        .expect("Failed to parse configuration file");

    config
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct RAGameResponse {
    title: String,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "ConsoleID")]
    console_id: u32,
    console_name: String,
    image_icon: String,
    num_achievements: u32,
    num_leaderboards: u32,
    points: u32,
    date_modified: String,

    #[serde(rename = "ForumTopicID")]
    forum_topic_id: u32,
    hashes: Vec<String>,
}

struct Rom<'a> {
    file_name: String,
    hash: String,
    matched_game: Option<&'a RAGameResponse>,
}

fn download_game_list_for_system(http_client: &reqwest::blocking::Client, api_key: &str, system_id: u32) -> Result<(), reqwest::Error> {
    let file_name = format!("data/system_games_{}.json", system_id);

    if std::path::Path::new(&file_name).exists() {
        println!("File {} already exists, skipping download.", file_name);
        return Ok(());
    }

    let hashes = "1";
    let only_with_achievements = "1";

    let api_url = &format!("https://retroachievements.org/API/API_GetGameList.php?i={}&h={}&f={}&y={}", system_id, hashes, only_with_achievements, api_key);
    
    let response = http_client.get(api_url).send()?.text()?;
    std::fs::write(&file_name, &response).expect("Unable to write file");

    return Ok(());
}

fn calculate_nds_file_hash(file_path: &str) -> String {
    let mut file = std::fs::File::open(file_path).expect("Unable to open file");

    let mut header = [0u8; 0x160];
    file.read_exact(&mut header).expect("Unable to read header");

    let icon_title_offset = u32::from_le_bytes(header[0x68..0x6C].try_into().expect("Invalid slice length")) as usize;
    let arm9_offset = u32::from_le_bytes(header[0x20..0x24].try_into().expect("Invalid slice length")) as usize;
    let arm9_size = u32::from_le_bytes(header[0x2C..0x30].try_into().expect("Invalid slice length")) as usize;

    let arm7_offset = u32::from_le_bytes(header[0x30..0x34].try_into().expect("Invalid slice length")) as usize;
    let arm7_size = u32::from_le_bytes(header[0x3C..0x40].try_into().expect("Invalid slice length")) as usize;

    let mut icon_title_data = vec![0u8; 0xA00];
    file.seek(SeekFrom::Start(icon_title_offset as u64)).expect("Unable to seek to icon/title data");
    file.read_exact(&mut icon_title_data).expect("Unable to read icon/title data");

    let mut arm9_data = vec![0u8; arm9_size];
    file.seek(SeekFrom::Start(arm9_offset as u64)).expect("Unable to seek to arm9 data");
    file.read_exact(&mut arm9_data).expect("Unable to read arm9 data");

    let mut arm7_data = vec![0u8; arm7_size];
    file.seek(SeekFrom::Start(arm7_offset as u64)).expect("Unable to seek to arm7 data");
    file.read_exact(&mut arm7_data).expect("Unable to read arm7 data");

    let mut hasher = md5::Context::new();
    hasher.consume(&header);
    hasher.consume(&arm9_data);
    hasher.consume(&arm7_data);
    hasher.consume(&icon_title_data);

    format!("{:x}", hasher.finalize())
}

fn main() {
    let user_config = get_user_config();

    std::fs::create_dir_all("data").expect("Unable to create data directory");

    let http_client = reqwest::blocking::Client::new();
    let system_id: u32 = 18; // DS

    match download_game_list_for_system(&http_client, user_config.api_key.as_str(), system_id) {
        Ok(_) => println!("Game list downloaded successfully."),
        Err(e) => eprintln!("Failed to download game list: {}", e),
    }

    let file_name = format!("data/system_games_{}.json", system_id);
    let file_content = std::fs::read_to_string(&file_name).expect("Unable to read file");
    let games: Vec<RAGameResponse> = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    let mut roms: Vec<Rom> = Vec::new();
    let game_titles: Vec<&str> = games.iter()
        .map(|game| game.title.as_str())
        .collect();
    
    for entry in std::fs::read_dir(user_config.roms_folder).expect("Unable to read roms folder") {
        let entry = entry.expect("Unable to read entry");
        let path = entry.path();
        
        if path.is_file() {
            let rom = Rom {
                file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
                hash: calculate_nds_file_hash(path.to_str().unwrap()),
                matched_game: None,
            };

            roms.push(rom);
        }
    }

    for mut rom in roms {
        println!("Checking ROM: {}", rom.file_name);

        let search_results: Vec<(&str, f32)> = fuzzy_search_best_n(&rom.file_name, &game_titles, 1);

        'search: for (title, score) in search_results {
            if score < 0.1 {
                continue;
            }

            println!("\tFound match: {} with score: {}", title, score);

            if let Some(game) = games.iter().find(|g| g.title == title) {
                println!("\tMatched game: {:?}", game);

                rom.matched_game = Some(game);
                
                println!("{}", rom.hash);

                for hash in &game.hashes {
                    if rom.hash == *hash {
                        println!("\tExact match found for game: {} with hash {}", title, hash);
                        break 'search;
                    }
                }
            } else {
                println!("\tNo exact match found for game: {}", title);
            }
        }
    }
}
