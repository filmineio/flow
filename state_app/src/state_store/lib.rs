use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncState {
    pub height: i64,
    pub block_cid: Option<String>,
    pub message_cid: Option<String>,
}