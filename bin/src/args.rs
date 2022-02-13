use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    RunNode {
        #[clap(short, long, help="Path to the config file", default_value_t=String::from("config.json"))]
        config: String,
    },

    GenerateClientJson {
        #[clap(short, long, help="Path to store the json")]
        save: Option<String>
    },

    // TODO: create transaction

    // TODO: get mine?
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}
