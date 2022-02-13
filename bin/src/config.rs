use serde::{Deserialize, Serialize};
use std::default::Default;
use std::error::Error;
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u32,
    pub address: String,
    pub data_file: String,
    pub trusted_nodes: Vec<String>,
}

impl Config {
    pub fn new(config_path: Option<&str>) -> Self {
        if let Some(path) = config_path {
            let c = Config::load(path).expect("Couldn't read config file!");
            log::info!("Loaded config from file!");
            c
        } else {
            log::info!("Loaded default config!");
            Config::default()
        }
    }

    pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        fs::write(path, serde_json::to_string(self)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 8033,
            address: "your_address".to_string(),
            data_file: "node.dat".to_string(),
            trusted_nodes: Vec::new(),
        }
    }
}
