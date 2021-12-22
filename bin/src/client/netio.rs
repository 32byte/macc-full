use super::{config::Config, data::Data};
use macc_lib::blockchain::{block::Block, blockchain::Blockchain, helper::SharedData};
use rocket::tokio;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone)]
pub struct NetIOModule {
    #[serde(skip)]
    client: reqwest::Client,

    #[serde(skip)]
    my_ip: String,
    #[serde(skip)]
    my_port: String,

    pub neighbors: Vec<String>,
}

impl NetIOModule {
    pub fn new() -> NetIOModule {
        NetIOModule {
            client: reqwest::Client::new(),
            my_ip: "".to_string(),
            my_port: "".to_string(),
            neighbors: Vec::new(),
        }
    }

    pub async fn init(&mut self, config: &Config) {
        self.client = reqwest::Client::new();

        let res = reqwest::get("https://api.ipify.org").await;

        self.my_ip = res
            .expect("Couldn't get self ip!")
            .text()
            .await
            .expect("Api didn't respond with sel ip!");

        self.my_port = config.port.to_string();
        log::info!("Self ip: {}", self.my_ip);

        for node in &config.trusted_nodes {
            if !self.neighbors.contains(node) {
                self.neighbors.push(node.clone());

                tokio::spawn(Self::request_register_node(
                    node.clone(),
                    format!("{}:{}", &self.my_ip, &self.my_port),
                ));
            }
        }
    }

    async fn request_register_node(
        node: String,
        my_ip: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("http://{}/register-self", node);

        log::warn!("Registering self on {}", &url);

        reqwest::Client::new()
            .post(url)
            .body(my_ip)
            .timeout(Duration::from_secs(5))
            .send()
            .await
    }

    async fn t_register_node(data: SharedData<Data>, node: String, my_ip: String) {
        log::debug!("Registering: {}", node);

        let res = Self::request_register_node(node.clone(), my_ip).await;

        if res.is_ok() {
            log::debug!("Adding {}", node);
            data.write().await.net_io.neighbors.push(node);
        } else {
            log::error!("{:#?}", res.unwrap_err());
        }
    }

    pub fn register_node(&self, node: String, data: SharedData<Data>) {
        if self.neighbors.contains(&node) {
            return;
        }

        tokio::spawn(Self::t_register_node(data, node, self.my_ip.clone()));
    }

    async fn t_send_block(node: String, block: Block, port: String) {
        let res = reqwest::Client::new()
            .post(format!("http://{}/new-block", node))
            .query(&[("port", &port)])
            .json(&block)
            .send()
            .await;

        if res.is_err() {
            log::warn!("Node {} seems to be offline!", node);
        }
    }

    pub fn broadcast(&self, block: &Block) {
        log::debug!("Broadcasting {:#?}!", self.neighbors);
        for node in &self.neighbors {
            tokio::spawn(Self::t_send_block(
                node.clone(),
                block.clone(),
                self.my_port.clone(),
            ));
        }
    }

    pub async fn get_blockchain(&self, node: &String) -> Option<Blockchain> {
        let res = reqwest::get(format!("http://{}/blockchain", node)).await;

        if let Ok(res) = res {
            if let Ok(body) = res.text().await {
                if let Ok(blockchain) = serde_json::from_str(&body) {
                    return Some(blockchain);
                }
            }
        }

        None
    }
}
