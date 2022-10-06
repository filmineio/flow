use lotus_rs::types::chain::block::Block;
use lotus_rs::types::chain::cid::CID;
use lotus_rs::types::chain::message::Message;
use lotus_rs::types::state::execution_trace::ExecutionTrace;
use lotus_rs::types::state::gas_charge::GasCharge;
use lotus_rs::types::state::message_rct::MessageRct;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlowMessageRct {
    pub ExitCode: Option<i8>,
    pub Return: Option<String>,
    pub GasUsed: Option<i64>,
    pub DecodedReturn: Option<Value>,
}

impl From<MessageRct> for FlowMessageRct {
    fn from(m: MessageRct) -> Self {
        FlowMessageRct {
            Return: m.Return,
            ExitCode: m.ExitCode,
            GasUsed: m.GasUsed,
            DecodedReturn: None,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlowBlock {
    pub Block: Block,
    pub Cid: Option<String>,
}

impl FlowBlock {
    pub fn set_cid(&mut self, cid: CID) {
        self.Cid = Some(cid["/"].clone())
    }
}

impl From<Block> for FlowBlock {
    fn from(value: Block) -> Self {
        Self {
            Block: value,
            Cid: None,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlowMessage {
    pub Cid: String,
    pub Message: Message,
    pub Height: Option<i64>,
    pub BlockCid: Option<String>,
    pub MessageRct: Option<FlowMessageRct>,
    pub DecodedParams: Option<Value>,
    pub GasCharges: Option<Vec<GasCharge>>,
    pub SubCallOf: Option<String>,
}

impl From<ExecutionTrace> for FlowMessage {
    fn from(exec_trace: ExecutionTrace) -> Self {
        Self {
            Cid: exec_trace.Msg.CID["/"].clone(),
            Message: exec_trace.Msg,
            Height: None,
            BlockCid: None,
            MessageRct: Some(FlowMessageRct::from(exec_trace.MsgRct)),
            DecodedParams: None,
            GasCharges: None, // ToDo if needed exec_trace.GasCharges,
            SubCallOf: None,
        }
    }
}

impl FlowMessage {
    pub fn set_block(&mut self, height: i64, block_cid: CID) {
        self.BlockCid = Some(block_cid["/"].clone());
        self.Height = Some(height);
    }

    pub fn set_decoded_params(&mut self, params: Value) {
        self.DecodedParams = Some(params);
        self.Message.Params = None;
    }

    pub fn set_sub_calls_of(&mut self, msg_cid: CID) {
        self.SubCallOf = Some(msg_cid["/"].clone())
    }
}
