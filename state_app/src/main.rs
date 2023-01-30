extern crate core;

use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use lotus_rs::client::LotusClient;
use lotus_rs::config::LotusConfig;
use serde_json::json;
use tokio::time::{Instant, sleep};

use crate::sync::sync;
use crate::sync_store::config::SyncStoreConfig;
use crate::sync_store::core::SyncStore;
use crate::types::{Bench, FlowMessage};

mod sync;
mod types;
mod sync_store;

#[tokio::main]
async fn main() -> Result<()> {
    let client = LotusClient::init(LotusConfig::from_env());
    let sync_store_cfg = SyncStoreConfig::from_env();
    let sync_store = SyncStore::new(sync_store_cfg)?;

    let mut map: HashMap<String, Option<String>> = HashMap::new();
    loop {
        let height = client.chain_head().await?.Height;
        let mut current_height = sync_store.get_current_height();
        while current_height < height {
            let now = Instant::now();
            if let Ok(d) = sync(&client, current_height, &mut map).await {
                let elapsed = now.elapsed();

                let b = Bench {
                    elapsed,
                    height: current_height,
                    target_height: height,
                };
                println!("{}", json!(b));
                current_height = d
            } else {
                current_height += 1;
            }

            sync_store.update_current_height(current_height);
            sleep(Duration::new(0, 0)).await;
        }
        sleep(Duration::new(35, 0)).await;
    }
}
