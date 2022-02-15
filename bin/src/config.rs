use serde::{Deserialize, Serialize};
use std::default::Default;
use std::error::Error;
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub address: String,
    pub data_file: String,
    pub trusted_nodes: Vec<String>,

    #[serde(skip)]
    path: String,
}

impl Config {
    pub fn new(path: &str) -> Self {
        if let Ok(config) = Config::load(path) {
            Config {
                path: path.to_string(),

                ..config
            }
        } else {
            log::warn!("Config does not exist, creating a new one!");

            Config {
                path: path.to_string(),

                ..Default::default()
            }
        }
    }

    pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn save(&self) -> std::io::Result<()> {
        fs::write(&self.path, serde_json::to_string(self)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 8033,
            address: "your_address".to_string(),
            data_file: "node.bin".to_string(),
            trusted_nodes: Vec::new(),

            path: "config.json".to_string(),
        }
    }
}
