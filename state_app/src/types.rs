use lotus_rs::client::LotusClient;
use lotus_rs::types::chain::block::Block;
use lotus_rs::types::chain::cid::{cid2str, CID};
use lotus_rs::types::chain::message::Message;
use lotus_rs::types::state::event::Entry;
use lotus_rs::types::state::execution_trace::ExecutionTrace;
use lotus_rs::types::state::gas_charge::GasCharge;
use lotus_rs::types::state::message_rct::MessageRct;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlowMessageRct {
    pub ExitCode: Option<i8>,
    pub Return: Option<String>,
    pub GasUsed: Option<i64>,
    pub EventsRoot: Option<String>,
}

impl From<MessageRct> for FlowMessageRct {
    fn from(m: MessageRct) -> Self {
        FlowMessageRct {
            Return: m.Return,
            ExitCode: m.ExitCode,
            GasUsed: m.GasUsed,
            EventsRoot: match m.EventsRoot {
                Some(v) => cid2str(v),
                _ => None,
            },
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
pub struct Addresses {
    pub RobustFrom: Option<String>,
    pub RobustTo: Option<String>,
    pub From: Option<String>,
    pub To: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlowMessage {
    pub Cid: String,
    pub Message: Message,
    pub Height: Option<i64>,
    pub BlockCid: Option<String>,
    pub MessageRct: Option<FlowMessageRct>,
    pub GasCharges: Option<Vec<GasCharge>>,
    pub SubCallOf: Option<String>,
    pub Addresses: Addresses,
    pub Value: Option<i64>,
    pub BlockTimestamp: Option<String>,
    pub NumberOfEvents: i64,
}

impl From<Message> for Addresses {
    fn from(value: Message) -> Self {
        let mut addr: Addresses = Addresses {
            From: None,
            RobustFrom: None,
            To: None,
            RobustTo: None,
        };

        if Addresses::is_robust(&value.From) {
            addr.RobustFrom = Some(value.From);
            addr.From = None;
        } else {
            addr.From = Some(value.From);
            addr.RobustFrom = None;
        }

        if Addresses::is_robust(&value.To) {
            addr.RobustTo = Some(value.To);
            addr.To = None;
        } else {
            addr.To = Some(value.To);
            addr.RobustTo = None;
        }

        addr
    }
}

impl Addresses {
    fn is_robust(addr: &str) -> bool {
        addr.len() > 10
    }

    async fn get_from_rpc(client: &LotusClient, address: &str) -> Option<String> {
        if Addresses::is_robust(address) {
            match client.state_lookup_id(address.to_string(), None).await {
                Ok(addr) => Some(addr),
                Err(_e) => None,
            }
        } else {
            match client
                .state_lookup_robust_address(address.to_string(), None)
                .await
            {
                Ok(addr) => Some(addr),
                Err(_e) => None,
            }
        }
    }

    async fn resolve_addr(
        client: &LotusClient,
        address: &str,
        map: &mut HashMap<String, Option<String>>,
    ) -> Option<String> {
        if let Some(add) = map.get(address) {
            return add.clone();
        }

        let addr = Addresses::get_from_rpc(client, address).await;
        map.insert(address.to_string(), addr.clone());

        addr
    }
}

impl From<ExecutionTrace> for FlowMessage {
    fn from(exec_trace: ExecutionTrace) -> Self {
        let mut val: i64 = 0;
        if let Some(v) = &exec_trace.Msg.Value {
            val = i64::from_str(v).unwrap_or(0)
        }

        Self {
            Cid: exec_trace.Msg.CID["/"].clone(),
            Message: exec_trace.Msg.clone(),
            Height: None,
            BlockCid: None,
            MessageRct: Some(FlowMessageRct::from(exec_trace.MsgRct)),
            GasCharges: None, // ToDo if needed exec_trace.GasCharges,
            SubCallOf: None,
            Addresses: Addresses::from(exec_trace.Msg),
            Value: Some(val),
            BlockTimestamp: None,
            NumberOfEvents: 0,
        }
    }
}

impl FlowMessage {
    pub fn set_block(&mut self, height: i64, block_cid: CID) {
        self.BlockCid = Some(block_cid["/"].clone());
        self.Height = Some(height);
    }

    pub fn set_sub_calls_of(&mut self, msg_cid: CID) {
        self.SubCallOf = Some(msg_cid["/"].clone())
    }

    pub fn set_number_of_events(&mut self, n: i64) {
        self.NumberOfEvents = n;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bench {
    pub elapsed: Duration,
    pub height: i64,
    pub target_height: i64,
}

impl FlowMessage {
    pub fn set_block_timestamp(&mut self, ts: String) {
        self.BlockTimestamp = Some(ts);
    }

    pub async fn resolve_addresses(
        &mut self,
        client: &LotusClient,
        map: &mut HashMap<String, Option<String>>,
    ) {
        if let Some(address) = &self.Addresses.From {
            self.Addresses.RobustFrom = Addresses::resolve_addr(client, address, map).await;
        }

        if let Some(address) = &self.Addresses.RobustFrom {
            self.Addresses.From = Addresses::resolve_addr(client, address, map).await;
        }

        if let Some(address) = &self.Addresses.To {
            self.Addresses.RobustTo = Addresses::resolve_addr(client, address, map).await;
        }

        if let Some(address) = &self.Addresses.RobustTo {
            self.Addresses.To = Addresses::resolve_addr(client, address, map).await;
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct FlowEvent {
    pub MessageCid: String,
    pub EventsRoot: String,
    pub Emitter: u64,
    pub Order: i32,
    pub Entries: Vec<Entry>,
}

#[allow(non_snake_case)]
impl From<(String, String, u64, i32, Vec<Entry>)> for FlowEvent {
    fn from(
        (MessageCid, EventsRoot, Emitter, Order, Entries): (String, String, u64, i32, Vec<Entry>),
    ) -> Self {
        FlowEvent {
            MessageCid,
            EventsRoot,
            Emitter,
            Order,
            Entries,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActorBls {
    pub Height: i64,
    pub Block: Option<String>,
    pub ActorId: String,
    pub Balance: i64,
    pub Processed: String,
}
