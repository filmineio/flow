use dotenv::dotenv;
use serde::{Deserialize, Serialize};

// TODO: Include flush interval and snapshot interval
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StateStoreConfig {
    pub path: String,
}

impl StateStoreConfig {
    pub fn from_env() -> StateStoreConfig {
        dotenv().unwrap();
        StateStoreConfig {
            path: std::env::var("STORE_PATH").expect("STORE_PATH is required"),
        }
    }

    pub fn new(path: String) -> StateStoreConfig {
        StateStoreConfig { path }
    }
}
