use dotenv::dotenv;
use serde::{Deserialize, Serialize};

// TODO: Include flush interval and snapshot interval
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncStoreConfig {
    pub path: String,
}

impl SyncStoreConfig {
    pub fn from_env() -> SyncStoreConfig {
        dotenv().unwrap();
        SyncStoreConfig {
            path: std::env::var("STORE_PATH").expect("STORE_PATH is required")
        }
    }

    pub fn new(path: String) -> SyncStoreConfig {
        SyncStoreConfig {
            path,
        }
    }
}
