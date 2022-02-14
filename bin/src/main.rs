use clap::Parser;
use log::{info, warn};
use macc_lib::{
    ecdsa::{create_rng, create_secp, pb_key_from_bytes, Client},
    hex::{FromHex, ToHex},
    PublicKey,
};
use tokio::runtime::Runtime;
use tokio::signal;

mod logger;
use logger::CustomLogger;

mod config;
use config::Config;

mod types;
use types::{share, Data};

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
        let data = if let Some(data) = Data::from_file(&config.data_file, config.clone()) {
            info!("Loaded node data from file!");
            data
        } else {
            warn!("Failed to load node data, creating new data!");
            Data::new(true, None, config.clone(), None, None, None, None, None)
        };

        let mining_data = (share(None), share(None));

        // spawn threads
        let h_worker = tokio::spawn(worker::start(data.clone(), mining_data.clone()));
        let h_miner = tokio::spawn(worker::start_miner(data.clone(), mining_data.clone()));

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

        data.save(&config.data_file)
            .expect("Couldn't save node data!");
        info!("Saved node data to file!");

        // save config
        // TODO: handle the modified version probably save somewhere else
        data.config.save().expect("Couldn't save config!");

        let _ = tokio::join!(h_worker, h_miner);
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

fn create_transaction(client_json: &str, vin: &str, vout: &str) {
    let mut client: Client = serde_json::from_str(
        &std::fs::read_to_string(client_json).expect("Could't find the client json!"),
    )
    .expect("Invalid client json!");

    println!(
        "{}",
        serde_json::to_string(&[("hello".to_string(), 2_usize)]).unwrap()
    );

    let vin: Vec<(String, usize)> = serde_json::from_str(vin).expect("Couldn't parse input!");
    let vin: Vec<([u8; 32], usize)> = vin
        .iter()
        .map(|(hex, index)| {
            let hash: [u8; 32] = Vec::from_hex(hex)
                .expect("Couldn't parse the hash as hex")
                .try_into()
                .expect("hash is the wrong lenght!");
            (hash, *index)
        })
        .collect();

    let vout: Vec<(u128, String)> = serde_json::from_str(vout).expect("Couldn't parse output!");
    let vout: Vec<(u128, PublicKey)> = vout
        .iter()
        .map(|(amount, pb_key)| {
            let pb_key =
                pb_key_from_bytes(&Vec::from_hex(pb_key).expect("Couldn't parse pb_key as hex!"))
                    .expect("Couldn't parse pb_key!");
            (*amount, pb_key)
        })
        .collect();

    let secp = create_secp();

    let tx = client
        .create_transaction(&secp, vin, vout)
        .expect("Couldn't create the transaction!");

    println!(
        "Transaction created with hash: {}",
        tx.hash().expect("Couldn't hash the transaction!").to_hex()
    );
    println!("");
    println!(
        "{}",
        serde_json::to_string(&tx).expect("Couldn't serialize the transaction!")
    );

    std::fs::write(
        client_json,
        &serde_json::to_string(&client).expect("Couldn't serialize client!"),
    )
    .expect("Couldn't update client json");
}

fn main() {
    let args = Args::parse();

    // set my logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .expect("Couldn't set the logger!");

    match &args.command {
        Command::RunNode { config } => start_node(config),
        Command::GenerateClientJson { save } => generate_client_json(save),
        Command::CreateTransaction {
            client_json,
            vin,
            vout,
        } => create_transaction(client_json, vin, vout),
    }
}
