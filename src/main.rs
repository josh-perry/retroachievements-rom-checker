use toml;
use tabled::{Table, settings::Style, Tabled};

use rust_fuzzy_search::{fuzzy_search_best_n};
use serde::{Deserialize, Serialize};

mod rom_hashes;

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
    hash: Option<String>,
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

#[derive(Tabled)]
struct TableRecord {
    #[tabled(rename = "ROM Name")]
    rom_name: String,

    #[tabled(rename = "Matched RA Game")]
    matched_game: String,

    #[tabled(rename = "Hash Match")]
    hash_status: String,
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
                hash: match rom_hashes::calculate_nds_file_hash(path.to_str().unwrap()) {
                    Ok(hash) => Some(hash),
                    Err(_) => None
                },
                matched_game: None,
            };

            roms.push(rom);
        }
    }

    for mut rom in &mut roms {
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
                
                for hash in &game.hashes {
                    if rom.hash == Some(hash.clone()) {
                        println!("\tExact match found for game: {} with hash {}", title, hash);
                        break 'search;
                    }
                }
            } else {
                println!("\tNo exact match found for game: {}", title);
            }
        }
    }

    let table_records: Vec<TableRecord> = roms.iter()
    .map(|rom| TableRecord {
        rom_name: rom.file_name.clone(),
        matched_game: match &rom.matched_game {
            Some(game) => format!("{} (ID: {})", game.title, game.id),
            None => "".to_string(),
        },
        hash_status: match &rom.hash {
            Some(_) => "✅".to_string(),
            None => "❌".to_string(),
        },
    }).collect();

    let mut table = Table::new(table_records);
    table.with(Style::modern());
    println!("{}", table);
}
