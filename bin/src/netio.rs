use std::error::Error;

use crate::{
    config::Config,
    types::{share, Shared},
};

use macc_lib::blockchain::{Block, Transaction, Blockchain};
use reqwest::Client;

pub struct NetIO {
    config: Shared<Config>,
}

impl NetIO {
    pub fn new(config: &Config) -> Self {
        Self {
            config: share(config.clone()),
        }
    }

    async fn get(&self, url: String) -> Option<String>
    {
        let res = reqwest::get(url).await.ok()?;
        res.text().await.ok()
    }

    async fn post(client: &Client, url: String, data: String) -> Result<u16, Box<dyn Error>> {
        let res = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(data.clone())
            .send()
            .await?;

        Ok(res.status().as_u16())
    }

    fn broadcast(&self, endpoint: String, data: String) {
        let config = self.config.clone();

        tokio::spawn(async move {
            let client = Client::new();
            let nodes = config
                .read()
                .expect("Couldn't read nodes!")
                .trusted_nodes
                .clone();
            let mut valid_nodes: Vec<String> = Vec::new();

            for (index, node) in nodes.iter().enumerate() {
                let url = format!("http://{}/{}", node, endpoint);
                debug!("Requesting {} with index {}", url, index);
                let status = NetIO::post(&client, url, data.clone()).await;

                if status.unwrap_or(400) != 200 {
                    warn!(
                        "\"{}\" is not responding or responding faulty, removing..",
                        node
                    );
                }// else {
                    // TODO: figure out something clever here
                    valid_nodes.push(node.clone());
                //}
            }

            (*config.write().expect("Couldn't write nodes!")).trusted_nodes = valid_nodes;
        });
    }

    pub fn b_block(&self, block: &Block, block_height: usize) -> Result<(), Box<dyn Error>> {
        debug!("Broadcasting block!");

        let data = serde_json::to_string(block)?;

        let endpoint = format!(
            "block?height={}&port={}",
            block_height,
            &self.config.read().expect("Couldn't read config!").port
        );

        self.broadcast(endpoint, data);

        Ok(())
    }

    pub fn b_transaction(&self, tx: &Transaction) -> Result<(), Box<dyn Error>> {
        debug!("Broadcasting transaction!");

        let data = serde_json::to_string(tx)?;

        let endpoint = "transaction".to_string();

        self.broadcast(endpoint, data);

        Ok(())
    }

    pub async fn get_blockchain(&self, node: &str) -> Option<Blockchain> {
        let url = format!("http://{}/blockchain", node);
        let bc_json = self.get(url).await?;

        serde_json::from_str(&bc_json).ok()
    }

    pub fn save(&self) -> std::io::Result<()> {
        self.config.read().expect("Couldn't read config!").save()
    }
}
