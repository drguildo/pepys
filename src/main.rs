use std::{
    collections::HashMap,
    env::args,
    io,
    path::{Path, PathBuf},
};

use chrono::{Date, Datelike, TimeZone, Utc};

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

    let arg = args().nth(1);
    let entry_date = if let Some(date_arg) = arg {
        let split: Vec<&str> = date_arg.split('-').collect();
        if split.len() != 3 {
            eprintln!("Invalid date");
            std::process::exit(-1);
        }
        let year = split[0].parse::<i32>().expect("Invalid year");
        let month = split[1].parse::<u32>().expect("Invalid month");
        let day = split[2].parse::<u32>().expect("Invalid day");
        if let chrono::LocalResult::Single(validated_date) = Utc.ymd_opt(year, month, day) {
            validated_date
        } else {
            eprintln!("Invalid date. Date should be in format YYYY-MM-DD and be valid.");
            std::process::exit(-1);
        }
    } else {
        Utc::now().date()
    };

    let diary_entry_path = get_diary_entry_path(&pepys_dir, &entry_date);
    if !diary_entry_path.exists() {
        if create_diary_entry(&diary_entry_path).is_ok() {
            println!("Created diary entry {}", diary_entry_path.to_str().unwrap());
        } else {
            eprintln!(
                "Failed to create diary entry {}",
                diary_entry_path.to_str().unwrap()
            );
            std::process::exit(-1);
        }
    }

    if edit::edit_file(&diary_entry_path).is_err() {
        eprintln!("Failed to spawn editor");
    }
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

fn get_diary_entry_path(diary_path: &Path, entry_date: &Date<Utc>) -> PathBuf {
    let mut diary_entry_path = diary_path.to_path_buf();

    diary_entry_path.push(format!("{:02}", entry_date.year()));
    diary_entry_path.push(format!("{:02}", entry_date.month()));
    diary_entry_path.push(format!("{:02}.txt", entry_date.day()));

    diary_entry_path
}

fn create_diary_entry(path: &Path) -> io::Result<std::fs::File> {
    let prefix = path
        .parent()
        .expect("Failed to get parent directory of diary entry path");
    std::fs::create_dir_all(prefix)?;
    std::fs::File::create(path)
}
