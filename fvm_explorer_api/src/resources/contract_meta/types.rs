use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractMetaPath {
    pub contract_address: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractMetadata {
    pub contract_address: String,
    pub abi_cid: String,
    pub main_cid: String,
    pub name: String,
    pub compiler_version: String,
    pub file_map: HashMap<String, String>,
}
