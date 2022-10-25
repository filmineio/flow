use crate::config::LotusConfig;
use crate::RPCRequest;
use reqwest::{Client, RequestBuilder, Response, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct LotusClient {
    config: LotusConfig,
    client: Client,
}

impl<'de> LotusClient {
    pub fn init(config: LotusConfig) -> LotusClient {
        LotusClient {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub fn build(&self) -> RequestBuilder {
        self.client
            .post(self.config.host_url.to_string())
            .bearer_auth(self.config.token.to_string())
            .header("Content-Type", self.config.content_type.to_string())
    }

    pub fn format<T: serde::Serialize>(&self, method: String, val: T) -> RPCRequest {
        RPCRequest {
            jsonrpc: "2.0".to_string(),
            method,
            id: 1,
            params: json!(val),
        }
    }

    pub async fn send<T: Deserialize<'static> + Debug>(
        &self,
        method: String,
        data: Vec<Value>,
    ) -> Result<Response> {
        self.build()
            .json(&self.format(format!("Filecoin.{}", method).to_string(), &data))
            .send()
            .await
    }
}
