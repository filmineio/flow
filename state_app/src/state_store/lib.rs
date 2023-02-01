use bincode;
use serde::{Deserialize, Serialize};
use sled;
use thiserror::Error;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncState {
    pub height: i64,
    pub block_cid: Option<String>,
    pub message_cid: Option<String>,
}


#[derive(Debug, Error)]
pub enum StateStoreError {
    #[error(transparent)]
    SledError(#[from] sled::Error),

    #[error(transparent)]
    BincodeError(#[from] bincode::Error),
}
