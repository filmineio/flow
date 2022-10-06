use crate::CID;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub Version: i32,
    pub To: String,
    pub From: String,
    pub Nonce: Option<i32>,
    pub Value: Option<String>,
    pub GasLimit: Option<i64>,
    pub GasFeeCap: Option<String>,
    pub GasPremium: Option<String>,
    pub Method: i8,
    pub Params: Option<String>,
    pub CID: CID,
}
