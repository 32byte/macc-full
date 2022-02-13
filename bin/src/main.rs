use tokio::runtime::Runtime;
use tokio::signal;

mod logger;
use logger::CustomLogger;

mod config;
use config::Config;

mod types;
use types::Data;

mod worker;

static LOGGER: CustomLogger = CustomLogger;
fn main() {
    // TODO: parse commandline args

    // set my logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .expect("Couldn't set the logger!");

    // create tokio runtime
    let rt: Runtime = Runtime::new().expect("Couldn't create tokio runtime!");

    rt.block_on(async {
        // create config
        // TODO: use path from args
        let config = Config::new(Some("config.json"));

        // create shared data
        // TODO: deserialize data
        let data = Data::new(true, None, config, None, None, None, None, None);

        // spawn threads
        let h_worker = tokio::spawn(worker::start(data.clone()));

        // TODO: server

        // handle ctrl+c
        match signal::ctrl_c().await {
            Ok(()) => {
                log::info!("Shutting down");

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

        let _ = tokio::join!(h_worker);
    });
}
