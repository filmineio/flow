use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Web3StorageConfig {
    pub token: String,
}
