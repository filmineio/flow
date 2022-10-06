extern crate core;

mod sync;
mod types;

use crate::sync::sync;
use crate::types::FlowMessage;
use anyhow::Result;
use lotus_rs::client::LotusClient;
use lotus_rs::config::LotusConfig;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    let client = LotusClient::init(LotusConfig::from_env());

    let mut current_height = client.chain_head().await?.Height;

    loop {
        sleep(Duration::new(4, 0)).await;
        let height = client.chain_head().await?.Height;
        while current_height < height {
            let d = sync(&client, current_height).await?;
            current_height = d;
        }
    }
}
