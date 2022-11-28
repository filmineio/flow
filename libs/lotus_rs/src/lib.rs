pub mod client;
pub mod config;
pub mod types;

use crate::client::LotusClient;

use crate::types::api::{RPCRequest, Rs};
use crate::types::chain::actor_state::ActorState;
use crate::types::chain::chain_head::ChainHead;
use crate::types::chain::cid::CID;
use crate::types::chain::message::Message;
use crate::types::state::event::StampedEvent;
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

    pub async fn chain_get_messages_in_tipset(&self, block_cid: CID) -> Result<Vec<Message>> {
        let res: Rs<Vec<Message>> = self
            .send::<Rs<Vec<Message>>>(
                "ChainGetMessagesInTipset".to_string(),
                vec![json!(vec![block_cid])],
            )
            .await?
            .json()
            .await?;

        Ok(res.result)
    }

    pub async fn state_decode_params(
        &self,
        to: String,
        method: i64,
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

    pub async fn state_lookup_robust_address(
        &self,
        address: String,
        ts: Option<CID>,
    ) -> Result<String> {
        let res: Rs<String> = self
            .send::<Rs<String>>(
                "StateLookupRobustAddress".to_string(),
                vec![json!(address), json!(vec![ts])],
            )
            .await?
            .json()
            .await?;

        Ok(res.result)
    }

    pub async fn state_lookup_id(&self, address: String, ts: Option<CID>) -> Result<String> {
        let res: Rs<String> = self
            .send::<Rs<String>>(
                "StateLookupID".to_string(),
                vec![json!(address), json!(vec![ts])],
            )
            .await?
            .json()
            .await?;

        Ok(res.result)
    }

    pub async fn state_get_actor(&self, address: String, ts: Option<CID>) -> Result<ActorState> {
        let res: Rs<ActorState> = self
            .send::<Rs<ActorState>>(
                "StateGetActor".to_string(),
                vec![json!(address), json!(vec![ts])],
            )
            .await?
            .json()
            .await?;

        Ok(res.result)
    }

    pub async fn chain_get_message(&self, message_cid: CID) -> Result<Message> {
        let res: Rs<Message> = self
            .send::<Rs<ActorState>>("ChainGetMessage".to_string(), vec![json!(message_cid)])
            .await?
            .json()
            .await?;

        Ok(res.result)
    }

    pub async fn chain_get_events(&self, events_root: CID) -> Result<Vec<StampedEvent>> {
        let res: Rs<Vec<StampedEvent>> = self
            .send::<Rs<Vec<StampedEvent>>>("ChainGetEvents".to_string(), vec![json!(events_root)])
            .await?
            .json()
            .await?;

        Ok(res.result)
    }
}
