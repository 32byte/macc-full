use std::net::SocketAddr;

use crate::types::Data;
use macc_lib::blockchain::{Block, Transaction};
use rocket::{serde::json::Json, State};

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct RawJson(String);

// GET
#[get("/blockchain?<start>&<stop>")]
fn get_blockchain(data: &State<Data>, start: Option<usize>, stop: Option<usize>) -> Option<RawJson> {
    let blockchain = &*data.blockchain.read().ok()?;
    let start = start.unwrap_or(0);
    let stop = stop.unwrap_or(blockchain.height());

    let json = serde_json::to_string(blockchain.slice(start, stop)?).ok()?;
    Some(RawJson(json))
}

#[get("/height")]
fn get_block_height(data: &State<Data>) -> Option<RawJson> {
    let json = serde_json::to_string(&(*data.blockchain.read().ok()?).height()).ok()?;
    Some(RawJson(json))
}
#[get("/txstore")]
fn get_tx_store(data: &State<Data>) -> Option<RawJson> {
    let json = serde_json::to_string(&*data.store.read().ok()?).ok()?;
    Some(RawJson(json))
}

// POST
#[post("/transaction", data = "<transaction>")]
fn post_transaction(data: &State<Data>, transaction: Json<Transaction>) -> Option<()> {
    data.i_transactions.write().ok()?.push(transaction.0);

    Some(())
}

#[post("/block?<height>&<port>", data = "<block>")]
fn post_block(
    data: &State<Data>,
    block: Json<Block>,
    height: usize,
    port: String,
    ip_addr: SocketAddr,
) -> Option<()> {
    data.i_blocks.write().ok()?.push((
        format!("{}:{}", ip_addr.ip().to_string(), port),
        height,
        block.0,
    ));

    Some(())
}

#[get("/")]
fn index() -> &'static str {
    "For a list of RESTAPI endpoints checkout https://github.com/32byte/macc-full/blob/master/docs/client-rest-api.md"
}

pub async fn start(data: Data) {
    // NICE-TO-HAVE: custom loglevel for rocket
    let config = rocket::Config {
        port: data.config.port,
        ..Default::default()
    };

    let _ = rocket::custom(config)
        .mount("/", routes![index])
        .mount("/", routes![get_blockchain, get_block_height, get_tx_store])
        .mount("/", routes![post_transaction, post_block])
        .manage(data)
        .launch()
        .await;
}
