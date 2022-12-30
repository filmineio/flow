use crate::CID;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockTicket {
    pub VRFProof: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ElectionProof {
    pub WinCount: i32,
    pub VRFProof: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BeaconEntry {
    pub Round: i32,
    pub Data: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WinPoStProof {
    pub PoStProof: i32,
    pub ProofBytes: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BLSAggregate {
    pub Type: i8,
    pub Data: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockSig {
    pub Type: i8,
    pub Data: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Block {
    pub Miner: String,
    pub Ticket: BlockTicket,
    pub ElectionProof: ElectionProof,
    pub BeaconEntries: Option<Vec<BeaconEntry>>,
    pub WinPoStProof: Option<Vec<WinPoStProof>>,
    pub Parents: Vec<CID>,
    pub ParentWeight: String,
    pub Height: i64,
    pub ParentStateRoot: CID,
    pub ParentMessageReceipts: CID,
    pub Messages: CID,
    pub BLSAggregate: Option<BLSAggregate>,
    pub Timestamp: i64,
    pub BlockSig: Option<BlockSig>,
    pub ForkSignaling: i64,
    pub ParentBaseFee: String,
}
