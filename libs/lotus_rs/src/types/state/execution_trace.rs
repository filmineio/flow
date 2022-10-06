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
pub struct ExecutionTrace {
    pub Msg: Message,
    pub MsgRct: MessageRct,
    pub Error: Option<String>,
    pub Duration: i64,
    pub Subcalls: Option<Vec<Self>>,
    pub GasCharges: Option<Vec<GasCharge>>,
}
