use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct KafkaConfig {
    pub connection_string: String,
    pub new_contract_topic: String,
}
