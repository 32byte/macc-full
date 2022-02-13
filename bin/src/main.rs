use macc_lib::ecdsa::{create_secp, create_rng, Client};
use tokio::runtime::Runtime;
use tokio::signal;
use clap::Parser;

mod logger;
use logger::CustomLogger;

mod config;
use config::Config;

mod types;
use types::Data;

mod args;
use args::{Args, Command};

mod worker;

static LOGGER: CustomLogger = CustomLogger;

fn start_node(config: &str) {
    // create tokio runtime
    let rt: Runtime = Runtime::new().expect("Couldn't create tokio runtime!");

    rt.block_on(async {
        // create config
        let config = Config::new(config);

        // create shared data
        // TODO: deserialize data
        let data = Data::new(true, None, config, None, None, None, None, None);

        // spawn threads
        let h_worker = tokio::spawn(worker::start(data.clone()));

        // TODO: server

        // handle ctrl+c
        match signal::ctrl_c().await {
            Ok(()) => {
                log::warn!("Shutting down");

                // shutdown
                *data
                    .running
                    .write()
                    .expect("Couldn't lock running for writing!") = false;
            }
            Err(_) => {
                log::error!("Shutting down");

                // shutdown
                *data
                    .running
                    .write()
                    .expect("Couldn't lock running for writing!") = false;
            }
        }

        // TODO: serialize and save data

        // save config
        // TODO: handle the modified version probably save somewhere else
        data.config
            .save()
            .expect("Couldn't save config!");

        let _ = tokio::join!(h_worker);
    });
}

fn generate_client_json(save: &Option<String>) {
    let secp = create_secp();
    let mut rng = create_rng().expect("Couldn't create OsRng!");

    let client = Client::new_random(&secp, &mut rng);

    let json = serde_json::to_string(&client).expect("Couldn't serialize client!");

    if let Some(path) = save {
        std::fs::write(path, &json).expect(&format!("Couldn't write to {}!", path));
        println!("Wrote client json to `{}`!", path);
    } else {
        println!("{}", json);
    }

}

fn main() {
    let args = Args::parse();

    // set my logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .expect("Couldn't set the logger!");

    match &args.command {
        Command::RunNode { config} => start_node(config),
        Command::GenerateClientJson { save} => generate_client_json(save)
    }
}
