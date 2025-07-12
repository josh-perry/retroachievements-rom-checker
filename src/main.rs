use std::{collections::HashMap, thread, time};
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct RASystemListResponse {
    #[serde(rename = "ID")]
    id: u32,
    name: String,

    #[serde(rename = "IconURL")]
    icon_url: String,
    active: bool,
    is_game_system: bool,
}

struct Rom<'a> {
    file_name: String,
    hash: Option<String>,
    matched_game: Option<&'a RAGameResponse>,
    hash_matches: bool,
    system_id: Option<u32>,
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

    thread::sleep(time::Duration::from_secs(2));
    return Ok(());
}

fn download_game_system_ids(http_client: &reqwest::blocking::Client, api_key: &str) -> Result<(), reqwest::Error> {
    let api_url = &format!("https://retroachievements.org/API/API_GetConsoleIDs.php?y={}", api_key);

    if std::path::Path::new("data/system_ids.json").exists() {
        println!("File data/system_ids.json already exists, skipping download.");
        return Ok(());
    }

    let response = http_client.get(api_url).send()?.text()?;
    std::fs::write("data/system_ids.json", &response).expect("Unable to write file");

    return Ok(());
}

#[derive(Tabled)]
struct TableRecord {
    #[tabled(rename = "Filename")]
    rom_name: String,

    system: String,

    #[tabled(rename = "RA game")]
    matched_game: String,

    #[tabled(rename = "Hash match?")]
    hash_status: String,
}

fn main() {
    let user_config = get_user_config();

    std::fs::create_dir_all("data").expect("Unable to create data directory");

    let http_client = reqwest::blocking::Client::new();

    // Download all system IDs
    match download_game_system_ids(&http_client, user_config.api_key.as_str()) {
        Ok(_) => println!("System IDs downloaded successfully."),
        Err(e) => eprintln!("Failed to download system IDs: {}", e),
    }

    // Read system IDs from the file
    let system_ids_file_content = std::fs::read_to_string("data/system_ids.json").expect("Unable to read system IDs file");
    let system_ids: Vec<RASystemListResponse> = serde_json::from_str(&system_ids_file_content).expect("Failed to parse system IDs JSON");

    // TODO: less hardcoding somehow.
    // The problem is that the folder names don't match the retroachievements system names.
    // For instance: the DS folder is called "nds" but the RA system name is "Nintendo DS".
    let system_name_to_id = HashMap::from([
        ("nds", 18),
        ("gba", 5),
        ("gbc", 6),
    ]);

    // Download game lists for each system
    for system in &system_ids {
        match download_game_list_for_system(&http_client, user_config.api_key.as_str(), system.id) {
            Ok(_) => println!("Game list downloaded successfully."),
            Err(e) => eprintln!("Failed to download game list: {}", e),
        }
    }

    let mut system_ids_to_games: HashMap<i32, Vec<RAGameResponse>> = HashMap::new();

    for system_id in system_name_to_id.values() {
        if !std::path::Path::new(&format!("data/system_games_{}.json", system_id)).exists() {
            eprintln!("Game list for system ID {} does not exist. Skipping.", system_id);
            continue;
        }

        let file_name = format!("data/system_games_{}.json", system_id);
        let file_content = std::fs::read_to_string(&file_name).expect("Unable to read file");
        let ra_game: Vec<RAGameResponse> = serde_json::from_str(&file_content).expect("Failed to parse JSON");

        system_ids_to_games.insert(*system_id as i32, ra_game);
    }

    let mut roms: Vec<Rom> = Vec::new();
    
    for entry in walkdir::WalkDir::new(user_config.roms_folder)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if path.is_file() {
            let system_string = path.parent().unwrap().file_name().unwrap().to_str().unwrap();

            let hashing_function =
            match rom_hashes::get_hash_function(system_string) {
                Some(func) => func,
                None => {
                    eprintln!("No hashing function found for system '{}'. Skipping file: {}", system_string, path.display());
                    continue;
                }
            };

            if system_name_to_id.get(system_string).is_none() {
                eprintln!("System '{}' is not supported. Skipping file: {}", system_string, path.display());
                continue;
            }

            let rom = Rom {
                file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
                hash: match hashing_function(path.to_str().unwrap()) {
                    Ok(hash) => Some(hash),
                    Err(_) => None,
                },
                matched_game: None,
                hash_matches: false,
                system_id: system_name_to_id.get(system_string).cloned(),
            };

            roms.push(rom);
        }
    }

     for rom in &mut roms {
        println!("Checking ROM: {}", rom.file_name);

        let system_games = match rom.system_id {
            Some(id) => system_ids_to_games.get(&(id as i32)),
            None => continue,
        };

        let game_titles: Vec<&str> = system_games.unwrap().iter()
            .map(|game| game.title.as_str())
            .collect();

        let search_results: Vec<(&str, f32)> = fuzzy_search_best_n(
            &rom.file_name,
            &game_titles,
            5,
        );

        'search: for (title, score) in search_results {
            if score < 0.4 {
                continue;
            }

            // TODO: make these skips configurable
            if title.contains("[Subset") || title.contains("~Hack~") || title.contains("~Homebrew~") {
                continue;
            }

            if let Some(game) = system_games.unwrap().iter().find(|g| g.title == title) {
                rom.matched_game = Some(&game);

                for hash in game.hashes.iter() {
                    if rom.hash == Some(hash.clone()) {
                        rom.hash_matches = true;
                    }
                }

                break 'search;
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
        hash_status: match rom.hash_matches {
            true => "✅".to_string(),
            false => "❌".to_string(),
        },
        system: match rom.system_id {
            Some(id) => {
                system_ids.iter().find(|system| system.id == id)
                    .map_or("Unknown".to_string(), |system| system.name.clone())
            },
            None => "Unknown".to_string(),
        }
    }).collect();

    let mut table = Table::new(table_records);
    table.with(Style::modern());
    println!("{}", table);
}
