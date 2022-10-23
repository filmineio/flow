use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ClickhouseConfig {
    pub connection_string: String,
}
