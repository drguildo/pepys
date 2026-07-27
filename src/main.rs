use std::{
    env::args,
    io,
    path::{Path, PathBuf},
};

use time::{macros::format_description, OffsetDateTime};

mod config;

fn main() {
    let config_path = config::get_config_path();

    let config = config::read_config(&config_path);
    let pepys_dir = if config.contains_key("diary_path") {
        std::path::PathBuf::from(config.get("diary_path").unwrap())
    } else {
        let mut dir = dirs::home_dir().expect("Failed to locate home directory");
        dir.push(config::DEFAULT_DIARY_PATH);
        dir
    };

    let arg = args().nth(1);
    let entry_date = if let Some(date_arg) = arg {
        let format = format_description!("[year]-[month]-[day]");
        if let Ok(validated_date) = time::OffsetDateTime::parse(&date_arg, &format) {
            validated_date
        } else {
            eprintln!("Invalid date. Date should be in format YYYY-MM-DD and be valid.");
            std::process::exit(-1);
        }
    } else {
        time::OffsetDateTime::now_utc()
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

fn get_diary_entry_path(diary_path: &Path, entry_date: &OffsetDateTime) -> PathBuf {
    let mut diary_entry_path = diary_path.to_path_buf();

    diary_entry_path.push(format!("{:02}", entry_date.year()));
    diary_entry_path.push(format!("{:02}", entry_date.month() as u8));
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
