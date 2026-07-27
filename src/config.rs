use std::{collections::HashMap, path::PathBuf};

pub(crate) const CONFIG_FILENAME: &str = "pepys.conf";
pub(crate) const DEFAULT_DIARY_PATH: &str = "pepys";

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
    let mut config = HashMap::new();

    if config_path.exists() {
        let config_contents =
            std::fs::read_to_string(config_path).expect("Failed to read config file");
        for line in config_contents.lines() {
            if line.contains(" = ") {
                let split_line = line.split_once(" = ").unwrap();
                config.insert(split_line.0.to_string(), split_line.1.to_string());
            }
        }
    }

    config
}
