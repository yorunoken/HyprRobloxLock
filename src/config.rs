use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, path::Path};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub screen_width: isize,
    pub screen_height: isize,
    pub monitor_x: isize,
    pub monitor_y: isize,
    pub edge_offset: isize,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let home_dir = env::var("HOME").expect("Failed to get HOME environment variable");
        let file_path = Path::new(&home_dir)
            .join(".config")
            .join("lock-roblox-cursor-hyprland")
            .join("Config.toml");

        let config_str = match fs::read_to_string(&file_path) {
            Ok(s) => s,
            Err(_) => panic!("No configuration file found, create one in `~/.config/lock-roblox-cursor-hyprland/Config.toml`")
        };
        let config = toml::from_str(&config_str)?;

        Ok(config)
    }
}
