use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct ContractMetaPath {
    pub contract_address: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContractMetadata {
    pub contract_address: String,
    pub abi_cid: String,
    pub main_cid: String,
    pub name: String,
    pub compiler_version: String,
    pub file_map: HashMap<String, String>,
    pub sig_cid: String,
    pub bin_cid: String,
}
