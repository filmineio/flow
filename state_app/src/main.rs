extern crate core;

use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use config::StateAppConfig;
use lotus_rs::client::LotusClient;
use lotus_rs::config::LotusConfig;
use serde_json::json;
use tokio::time::{sleep, Instant};

use crate::state_store::{config::StateStoreConfig, core::StateStore};
use crate::sync::sync;
use crate::types::{Bench, FlowMessage};

mod config;
mod state_store;
mod sync;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let config = StateAppConfig::from_env();
    let client = LotusClient::init(LotusConfig::from_env());
    let state_store_cfg = StateStoreConfig::from_env();
    let state_store = StateStore::new(state_store_cfg.clone())?;
    state_store
        .update_initial_height(config.initial_height.clone(), state_store_cfg.reset.clone())
        .await?;

    let mut map: HashMap<String, Option<String>> = HashMap::new();
    loop {
        let height = client.chain_head().await?.Height;
        let mut current_height = state_store.get_current_height().await;
        while current_height < height {
            let now = Instant::now();
            if let Ok(d) = sync(&client, current_height, &mut map).await {
                let elapsed = now.elapsed();

                let b = Bench {
                    elapsed,
                    height: current_height,
                    target_height: height,
                    error: None
                };
                println!("{}", json!(b));
                current_height = d
            } else {
                let elapsed = now.elapsed();

                let b = Bench {
                    elapsed,
                    height: current_height,
                    target_height: height,
                    error: Some("Failed to process Height".to_string())
                };
                println!("{}", json!(b));

                current_height += 1;
            }

            state_store.update_current_height(current_height).await?;
            sleep(Duration::new(2, 0)).await;
        }
        sleep(Duration::new(35, 0)).await;
    }
}
