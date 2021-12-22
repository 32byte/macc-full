use super::{config::Config, data::Data, node::Module};
use macc_lib::blockchain::helper::SharedData;
use rocket::tokio::{self, task::JoinHandle};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

pub struct ServerModule;

mod server_routes {
    use std::net::SocketAddr;

    use macc_lib::blockchain::blockchain::BlockChainMethods;
    use macc_lib::blockchain::{
        block::Block, blockchain::Blockchain, helper::SharedData, transaction::Transaction,
        txstore::TxStore,
    };
    use rocket::{serde::json::Json, State};
    use crate::client::data::Data;

    #[get("/block-height")]
    pub async fn get_block_height(data: &State<SharedData<Data>>) -> String {
        data.read().await.blockchain.height().to_string()
    }

    #[get("/blockchain?<from>&<until>")]
    pub async fn get_blockchain(
        data: &State<SharedData<Data>>,
        from: Option<usize>,
        until: Option<usize>,
    ) -> Json<Blockchain> {
        let blockchain = data.read().await.blockchain.clone();
        let from = if let Some(f) = from { f } else { 0 };
        let until = if let Some(u) = until {
            u
        } else {
            blockchain.len()
        };
        Json((&blockchain[from..until]).to_vec())
    }

    #[get("/neighbors")]
    pub async fn get_neighbors(data: &State<SharedData<Data>>) -> Json<Vec<String>> {
        Json(data.read().await.net_io.neighbors.clone())
    }

    #[get("/tx-store")]
    pub async fn get_tx_store(data: &State<SharedData<Data>>) -> Json<TxStore> {
        Json(data.read().await.tx_store.clone())
    }

    #[get("/difficulty")]
    pub async fn get_difficulty(data: &State<SharedData<Data>>) -> Json<[u8; 32]> {
        Json(data.read().await.blockchain.block_at(-1).difficulty)
    }

    #[get("/mempool")]
    pub async fn get_mempool(data: &State<SharedData<Data>>) -> Json<Vec<Transaction>> {
        Json(data.read().await.new_transactions.transactions.keys().cloned().collect())
    }

    #[post("/new-tx", data = "<tx>")]
    pub async fn new_transaction(data: &State<SharedData<Data>>, tx: Json<Transaction>) -> String {
        log::debug!("Added transaction to verify: {:?}", tx);

        data.write().await.new_transactions.add_tx(&tx.into_inner());

        "Transaction pending for verification!".to_string()
    }

    #[post("/new-block?<port>", data = "<block>")]
    pub async fn new_block(
        remote_addr: SocketAddr,
        data: &State<SharedData<Data>>,
        block: Json<Block>,
        port: String,
    ) -> String {
        log::debug!("Received block!");

        let remote = format!("{}:{}", remote_addr.ip(), port);

        log::error!("Block from remote: {}", remote);

        data.write()
            .await
            .new_blocks
            .insert(remote, vec![block.into_inner()]);

        "Block pending for verification".to_string()
    }

    #[post("/register-self", data = "<node>")]
    pub async fn register_self(data: &State<SharedData<Data>>, node: String) -> String {
        log::debug!("Received registration request for {}!", node);
        data.write()
            .await
            .net_io
            .register_node(node, data.inner().clone());
        log::debug!("After receiving registration!");

        "Registration pending!".to_string()
    }

    #[catch(404)]
    pub fn not_found() -> String {
        "Couldn't find page!".to_string()
    }

    #[get("/")]
    pub fn index(remote_addr: SocketAddr) -> String {
        format!("Hello there! :) {:?}", remote_addr)
    }
}

impl ServerModule {
    async fn start(config: Config, data: SharedData<Data>) {
        use server_routes::*;

        // setup tokio server
        let figment = rocket::Config::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", config.port))
            .merge(("log", "off"));

        let _ = rocket::custom(figment)
            .mount("/", routes![index])
            .mount(
                "/",
                routes![
                    get_block_height,
                    get_blockchain,
                    get_neighbors,
                    get_tx_store,
                    get_difficulty,
                    get_mempool
                ],
            )
            .mount("/", routes![new_transaction, new_block, register_self])
            .register("/", catchers![not_found])
            .manage(data)
            .attach(CORS)
            .launch()
            .await;
    }
}

impl Module for ServerModule {
    fn start_thread(&self, config: &Config, data: SharedData<Data>) -> JoinHandle<()> {
        tokio::spawn(ServerModule::start(config.clone(), data))
    }
}
