use crate::types::state::gas_charge::GasCharge;
use crate::types::state::message_rct::MessageRct;
use crate::{Message, CID};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InvocResult {
    pub Msg: Message,
    pub Duration: i64,
    pub MsgCid: CID,
    pub MsgRct: MessageRct,
    pub ExecutionTrace: ExecutionTrace,
    pub Error: Option<String>,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExecutionTraceMessage {
    pub Version: Option<i32>,
    pub To: String,
    pub From: String,
    pub Nonce: Option<i32>,
    pub Value: Option<String>,
    pub GasLimit: Option<i64>,
    pub GasFeeCap: Option<String>,
    pub GasPremium: Option<String>,
    pub Method: i64,
    pub Params: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExecutionTrace {
    pub Msg: ExecutionTraceMessage,
    pub MsgRct: MessageRct,
    pub Error: Option<String>,
    pub Subcalls: Option<Vec<Self>>,
    pub GasCharges: Option<Vec<GasCharge>>,
}
