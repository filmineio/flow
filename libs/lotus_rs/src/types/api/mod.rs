pub mod core;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rs<T> {
    pub result: T,
    pub jsonrpc: String,
    pub id: i64,
}

#[derive(Serialize)]
pub struct RPCRequest {
    pub jsonrpc: String,
    pub method: String,
    pub id: i64,
    pub params: serde_json::Value,
}
