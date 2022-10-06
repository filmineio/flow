pub mod client;
pub mod config;
pub mod types;

use crate::client::LotusClient;

use crate::types::api::{RPCRequest, Rs};
use crate::types::chain::block_messages::BlockMessages;
use crate::types::chain::chain_head::ChainHead;
use crate::types::chain::cid::CID;
use crate::types::chain::message::Message;
use crate::types::state::state::State;
use anyhow::Result;
use serde_json::Value::Null;
use serde_json::{json, Value};

impl LotusClient {
    pub async fn chain_head(&self) -> Result<ChainHead> {
        let res: Rs<ChainHead> = self
            .send::<Rs<ChainHead>>("ChainHead".to_string(), vec![])
            .await?
            .json()
            .await?;

        Ok(res.result)
    }

    pub async fn chain_get_tip_set_by_height(&self, height: i64) -> Result<ChainHead> {
        let res: Rs<ChainHead> = self
            .send::<Rs<ChainHead>>(
                "ChainGetTipSetByHeight".to_string(),
                vec![Value::from(height), Null],
            )
            .await?
            .json()
            .await?;

        Ok(res.result)
    }

    pub async fn state_compute(
        &self,
        height: i64,
        messages: Vec<Message>,
        tip_sets: Vec<CID>,
    ) -> Result<State> {
        let res: Rs<State> = self
            .send::<Rs<State>>(
                "StateCompute".to_string(),
                vec![Value::from(height), json!(messages), json!(tip_sets)],
            )
            .await?
            .json()
            .await?;

        Ok(res.result)
    }

    pub async fn get_block_messages(&self, block_cid: CID) -> Result<BlockMessages> {
        let res: Rs<BlockMessages> = self
            .send::<Rs<BlockMessages>>("ChainGetBlockMessages".to_string(), vec![json!(block_cid)])
            .await?
            .json()
            .await?;

        Ok(res.result)
    }

    pub async fn state_decode_params(
        &self,
        to: String,
        method: i8,
        params: String,
    ) -> Result<Value> {
        let res: Rs<Value> = self
            .send::<Rs<Value>>(
                "StateDecodeParams".to_string(),
                vec![json!(to), json!(method), json!(params), Null],
            )
            .await?
            .json()
            .await?;

        Ok(res.result)
    }
}
