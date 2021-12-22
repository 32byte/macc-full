#[macro_use]
extern crate rocket;

mod client;

use client::{logger::CustomLogger, node::Node, server::ServerModule, worker::WorkerModule};

static LOGGER: CustomLogger = CustomLogger;

#[rocket::main]
async fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .expect("Couldn't set the logger!");
    
    Node::start(Some("config.json"), WorkerModule, ServerModule).await;
}
