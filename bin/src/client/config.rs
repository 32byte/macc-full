use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Clone)]
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

    pub fn load(file: &str) -> Result<Config, Box<dyn Error>> {
        Ok(serde_json::from_str(&std::fs::read_to_string(file)?)?)
    }
}

impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            port: 8033,
            address: "node".to_string(),
            data_file: "node.dat".to_string(),
            trusted_nodes: Vec::new(),
        }
    }
}
