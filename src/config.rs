use std::{collections::HashMap, path::PathBuf};

const CONFIG_FILENAME: &str = "pepys.conf";
const DEFAULT_DIARY_PATH: &str = "pepys";

pub fn get_config_path() -> PathBuf {
    let mut config_path = dirs::config_dir().expect("Failed to find user configuration directory");
    config_path.push(CONFIG_FILENAME);

    println!(
        "{}: {}",
        stringify!(CONFIG_FILENAME),
        config_path.to_str().unwrap()
    );

    config_path
}

pub fn read_config(config_path: &PathBuf) -> HashMap<String, String> {
    let mut config = get_default_config();

    if config_path.exists() {
        let config_contents =
            std::fs::read_to_string(config_path).expect("Failed to read config file");
        for line in config_contents.lines() {
            if line.contains(" = ") {
                if let Some(split_line) = line.split_once(" = ") {
                    config.insert(split_line.0.to_string(), split_line.1.to_string());
                } else {
                    eprintln!("Invalid config line: {}", line);
                }
            }
        }
    }

    config
}

fn get_default_config() -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert("diary_path".to_string(), DEFAULT_DIARY_PATH.to_string());
    config
}
