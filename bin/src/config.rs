use macc_lib::ecdsa::{Client, create_secp, create_rng};
use macc_lib::{Secp256k1, All};
use std::default::Default;

// TODO: custom (de)-serializing
#[derive(Clone)]
pub struct Config {
    pub port: u32,
    pub address: String,
    pub data_file: String,
    pub trusted_nodes: Vec<String>,

    pub secp: Secp256k1<All>,
    pub miner_client: Client,
}

impl Config {
    pub fn new(_config_path: Option<&str>) -> Self {
        // if let Some(path) = config_path {
        //     let c = Config::load(path).expect("Couldn't read config file!");
        //     log::info!("Loaded config from file!");
        //     c
        // } else {
        //     log::info!("Loaded default config!");
        // }
        Config::default()
    }

    // pub fn load(file: &str) -> Result<Config, Box<dyn Error>> {
    //     Ok(serde_json::from_str(&std::fs::read_to_string(file)?)?)
    // }

    // TODO: config saving
}

impl Default for Config {
    fn default() -> Self {
        let secp = create_secp();
        let mut rng = create_rng().expect("Couldn't create OsRNG");

        Config {
            port: 8033,
            address: "node".to_string(),
            data_file: "node.dat".to_string(),
            trusted_nodes: Vec::new(),

            miner_client: Client::new_random(&secp, &mut rng),
            secp: secp,
        }
    }
}
