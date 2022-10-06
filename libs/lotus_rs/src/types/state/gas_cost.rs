use crate::CID;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GasCost {
    pub Message: CID,
    pub GasUsed: Option<u64>,
    pub BaseFeeBurn: Option<u64>,
    pub OverEstimationBurn: Option<u64>,
    pub MinerPenalty: Option<u64>,
    pub MinerTip: Option<u64>,
    pub Refund: Option<u64>,
    pub TotalCost: Option<u64>,
}
