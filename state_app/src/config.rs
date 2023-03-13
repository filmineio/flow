use dotenv::dotenv;
use serde::{Deserialize, Serialize};

// TODO: Include flush interval and snapshot interval
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StateAppConfig {
    pub initial_height: i64,
}

impl StateAppConfig {
    pub fn from_env() -> StateAppConfig {
        dotenv().unwrap();
        StateAppConfig {
            initial_height: std::env::var("INITIAL_HEIGHT")
                .unwrap_or("0".to_string())
                .parse()
                .unwrap_or(0),
        }
    }
}
