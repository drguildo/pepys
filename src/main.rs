use std::{collections::HashMap, path::PathBuf};

use chrono::Datelike;

const CONFIG_FILENAME: &str = "pepys.conf";
const DEFAULT_DIARY_PATH: &str = "pepys";

fn main() {
    let config = read_config();
    let pepys_dir = if config.contains_key("diary_path") {
        std::path::PathBuf::from(config.get("diary_path").unwrap())
    } else {
        let mut dir = dirs::home_dir().expect("Failed to locate home directory");
        dir.push(DEFAULT_DIARY_PATH);
        dir
    };

    let diary_entry_path = get_diary_entry_path(&pepys_dir);
    println!("{:?}", diary_entry_path);
}

fn read_config() -> HashMap<String, String> {
    let mut config = HashMap::new();

    let mut config_path = dirs::config_dir().expect("Failed to find user configuration directory");
    config_path.push(CONFIG_FILENAME);

    if config_path.exists() {
        let config_contents =
            std::fs::read_to_string(&config_path).expect("Failed to read config file");
        for line in config_contents.lines() {
            if line.contains(" = ") {
                let split_line = line.split_once(" = ").unwrap();
                config.insert(split_line.0.to_string(), split_line.1.to_string());
            }
        }
    }

    config
}

fn get_diary_entry_path(pepys_dir: &PathBuf) -> PathBuf {
    let mut diary_entry_path = pepys_dir.clone();

    let now = chrono::Utc::now();
    diary_entry_path.push(format!("{:02}", now.year()));
    diary_entry_path.push(format!("{:02}", now.month()));
    diary_entry_path.push(format!("{:02}.txt", now.day()));

    diary_entry_path
}
