use crate::types::chain::block::Block;
use crate::CID;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChainHead {
    pub Height: i64,
    pub Blocks: Vec<Block>,
    pub Cids: Vec<CID>,
}
