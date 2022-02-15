use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Subcommand, Debug)]
pub enum Command {
    RunNode {
        #[clap(short, long, help="Path to the config file", default_value_t=String::from("config.json"))]
        config: String,
    },

    GenerateClientJson {
        #[clap(short, long, help = "Path to store the json")]
        save: Option<String>,
    },

    CreateTransaction {
        #[clap(short, long, help = "Path to the client json")]
        client_json: String,

        #[clap(long, help = "Input in json format")]
        vin: String,

        #[clap(long, help = "Output in json format")]
        vout: String,
    },
    // TODO: get mine?
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long, help = "Sets the log level")]
    pub log_level: Option<LevelFilter>,

    #[clap(subcommand)]
    pub command: Command,
}
