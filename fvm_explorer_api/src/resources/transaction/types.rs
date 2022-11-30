use clickhouse_rs::types::{Complex, Row};
use serde::{Deserialize, Serialize};

use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub current_bls: i64,
    pub next_bls: i64,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub cid: String,
    pub height: i64,
    pub block: String,
    pub message_rct_return: String,
    pub message_rct_exit_code: i64,
    pub message_rct_gas_used: i64,
    pub message_rct_events_root: String,
    pub sub_call_of: String,
    pub from: String,
    pub to: String,
    pub robust_from: String,
    pub robust_to: String,
    pub gas_limit: i64,
    pub gas_fee_cap: String,
    pub gas_premium: String,
    pub method: i64,
    pub params: String,
    pub value: i64,
    pub timestamp: i64,
    pub nonce: i64,
    pub state_transition: Option<StateTransition>,
    pub version: Option<i64>,
    pub number_of_events: i64,
}

impl Transaction {
    pub fn set_state_transition(
        mut tx: Transaction,
        me: String,
        current_bls: i64,
    ) -> (Transaction, i64) {
        let mut next_state = StateTransition {
            current_bls,
            next_bls: current_bls,
            from: tx.from.clone(),
            to: tx.to.clone(),
        };

        if tx.message_rct_exit_code == 0 {
            if tx.from == me || tx.robust_from == me {
                next_state.next_bls = next_state.current_bls - tx.value;
            } else {
                next_state.next_bls = next_state.current_bls + tx.value;
            }
        }

        tx.state_transition = Some(next_state.clone());

        (tx, next_state.next_bls)
    }
}

impl FromRow<Transaction> for Transaction {
    fn from_row(row: Row<Complex>) -> anyhow::Result<Self> {
        let mut c = Self::default();

        c.cid = row.get("Cid")?;
        c.height = row.get("Height")?;
        c.block = row.get("Block")?;
        c.message_rct_exit_code = row.get("MessageRctExitCode")?;
        c.message_rct_return = row.get("MessageRctReturn")?;
        c.message_rct_gas_used = row.get("MessageRctGasUsed")?;
        c.message_rct_events_root = row.get("MessageRctEventsRoot")?;
        c.sub_call_of = row.get("SubCallOf")?;
        c.from = row.get("From")?;
        c.to = row.get("To")?;
        c.robust_from = row.get("RobustFrom")?;
        c.robust_to = row.get("RobustTo")?;
        c.gas_limit = row.get("GasLimit")?;
        c.gas_fee_cap = row.get("GasFeeCap")?;
        c.gas_premium = row.get("GasPremium")?;
        c.method = row.get("Method")?;
        c.params = row.get("Params")?;
        c.value = row.get("Value")?;
        c.timestamp = row.get("Timestamp")?;
        c.nonce = row.get("Nonce")?;
        c.number_of_events = row.get("NumberOfEvents")?;

        Ok(c)
    }
}

impl ApiResource for Transaction {
    fn get_table() -> String {
        "flow.messages".to_string()
    }

    fn default_order_by() -> String {
        "BlockTimestamp".to_string()
    }

    fn default_search_by() -> String {
        "".to_string()
    }

    fn match_order_by(order_by: String) -> String {
        match order_by.to_lowercase().as_str() {
            "height" => "Height".to_string(),
            _ => "Timestamp".to_string(),
        }
    }

    fn match_search_by(search: String) -> Vec<String> {
        match search.to_lowercase().as_str() {
            "contract" => vec![
                "From".to_string(),
                "To".to_string(),
                "RobustFrom".to_string(),
                "RobustTo".to_string(),
            ],
            "subcalls" => vec!["SubCallOf".to_string()],
            "block" => vec!["Block".to_string()],
            _ => vec!["Cid".to_string()],
        }
    }
}

#[derive(Deserialize)]
pub struct DecodeParamsBody {
    pub to: String,
    pub method: i64,
    pub params: String,
}
