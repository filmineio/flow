extern crate core;

mod sync;
mod types;

use crate::sync::sync;
use crate::types::{Bench, FlowMessage};
use anyhow::Result;
use lotus_rs::client::LotusClient;
use lotus_rs::config::LotusConfig;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::{sleep, Instant};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LotusClient::init(LotusConfig::from_env());

    let mut current_height = 6265;
    let mut map: HashMap<String, Option<String>> = HashMap::new();
    loop {
        let height = client.chain_head().await?.Height;
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
            sleep(Duration::new(0, 0)).await;
        }
        sleep(Duration::new(30, 0)).await;
    }
}
