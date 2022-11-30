use anyhow::Result;
use dotenv::dotenv;
use serde::Deserialize;
use serde_with::with_prefix;

use crate::shared::app_config::ch_config::ClickhouseConfig;
use crate::shared::app_config::kafka_config::KafkaConfig;
use crate::shared::app_config::pg_config::PostgresConfig;
use crate::shared::app_config::server_config::ServerConfig;
use crate::shared::app_config::web3_storage_config::Web3StorageConfig;

with_prefix!(prefix_server "server.");
with_prefix!(prefix_pg "pg.");
with_prefix!(prefix_clickhouse "clickhouse.");
with_prefix!(prefix_kafka "kafka.");
with_prefix!(prefix_w3s "w3s.");

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(flatten, with = "prefix_server")]
    pub server: ServerConfig,
    #[serde(flatten, with = "prefix_pg")]
    pub pg: PostgresConfig,
    #[serde(flatten, with = "prefix_clickhouse")]
    pub ch: ClickhouseConfig,
    #[serde(flatten, with = "prefix_kafka")]
    pub broker: KafkaConfig,
    #[serde(flatten, with = "prefix_w3s")]
    pub w3s: Web3StorageConfig,
}

impl AppConfig {
    pub fn init() -> Result<Self> {
        dotenv()?;
        let config: AppConfig = envy::from_env::<AppConfig>()?;
        Ok(config)
    }
}
