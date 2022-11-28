use crate::shared::listener::contract_type::ContractType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContractTransaction {
    pub cid: Option<String>,
    pub height: Option<i64>,
    pub block: Option<String>,
    pub message_rct_exit_code: Option<i64>,
    pub message_rct_return: Option<String>,
    pub message_rct_gas_used: Option<i64>,
    pub sub_call_of: Option<String>,
    pub from: Option<String>,
    pub robust_from: Option<String>,
    pub robust_to: Option<String>,
    pub eth_address: Option<String>,
    pub to: Option<String>,
    pub value: Option<i64>,
    pub gas_limit: Option<i64>,
    pub gas_fee_cap: Option<String>,
    pub gas_premium: Option<String>,
    pub method: Option<i64>,
    pub params: Option<String>,
    pub timestamp: Option<String>,
    pub nonce: Option<i64>,
    pub contract_type: ContractType,
    pub version: Option<i32>,
}
