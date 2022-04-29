use std::collections::HashMap;

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
    println!("{:?}", &pepys_dir);
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
