use crate::shared::encoding::json::de_u16_from_str;
use crate::shared::logger::types::LoggerFormat;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(deserialize_with = "de_u16_from_str")]
    pub port: u16,
    pub logger_format: LoggerFormat,
}
