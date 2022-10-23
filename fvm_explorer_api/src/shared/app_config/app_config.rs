use crate::shared::app_config::ch_config::ClickhouseConfig;
use crate::shared::app_config::kafka_config::KafkaConfig;
use crate::shared::app_config::pg_config::PostgresConfig;
use crate::shared::app_config::server_config::ServerConfig;
use anyhow::Result;
use dotenv::dotenv;
use serde::Deserialize;
use serde_with::with_prefix;

with_prefix!(prefix_server "server.");
with_prefix!(prefix_pg "pg.");
with_prefix!(prefix_clickhouse "clickhouse.");
with_prefix!(prefix_kafka "kafka.");

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
}

impl AppConfig {
    pub fn init() -> Result<Self> {
        dotenv()?;
        let config: AppConfig = envy::from_env::<AppConfig>()?;
        Ok(config)
    }
}
