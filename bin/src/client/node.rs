use super::{config::Config, data::Data};
use macc_lib::blockchain::helper::SharedData;
use rocket::tokio::{self, signal, sync::RwLock, task::JoinHandle};

pub trait Module {
    fn start_thread(&self, config: &Config, data: SharedData<Data>) -> JoinHandle<()>;
}

pub struct Node;

impl Node {
    pub async fn start<W, S>(config_path: Option<&str>, worker_module: W, server_module: S)
    where
        W: Module,
        S: Module,
    {
        let config = Config::new(config_path);
        let mut data: Data = Data {
            running: true,
            ..Data::new(&config).await
        };
        data.net_io.init(&config).await;

        // setup shared data
        let shared_data = SharedData::new(RwLock::new(data));

        // setup thread handles
        let server_handle = server_module.start_thread(&config, shared_data.clone());
        let worker_handle = worker_module.start_thread(&config, shared_data.clone());

        // listen for ctrl + c
        match signal::ctrl_c().await {
            Ok(()) => {
                log::info!("Pressed CTRL+C, stopping..");
                shared_data.write().await.running = false;
            }
            Err(_) => {
                log::error!("Coun't await CTRL+C!");
                shared_data.write().await.running = false;
            }
        }

        shared_data.read().await.save(&config);

        // wait for the threads to stop
        let _ = tokio::join!(worker_handle, server_handle);
    }
}
