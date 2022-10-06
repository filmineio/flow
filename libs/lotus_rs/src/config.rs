use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LotusConfig {
    pub host_url: String,
    pub content_type: String,
    pub token: String,
}

impl LotusConfig {
    pub fn from_env() -> LotusConfig {
        dotenv().unwrap();
        LotusConfig {
            host_url: std::env::var("LOTUS_API_HOST_URL").expect("LOTUS_API_HOST_URL is required"),
            content_type: std::env::var("LOTUS_HTTP_CONTENT_TYPE")
                .unwrap_or("application/json".to_string()),
            token: std::env::var("LOTUS_API_TOKEN").unwrap_or("".to_string()),
        }
    }

    pub fn new(
        host_url: String,
        content_type: Option<String>,
        token: Option<String>,
    ) -> LotusConfig {
        LotusConfig {
            host_url,
            content_type: content_type.unwrap_or("application/json".to_string()),
            token: token.unwrap_or("".to_string()),
        }
    }
}
