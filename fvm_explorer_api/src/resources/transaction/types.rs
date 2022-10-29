use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;
use clickhouse_rs::types::{Complex, Row};
use serde::{Deserialize, Serialize};

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

        if tx.from == me || tx.robust_from == me {
            next_state.next_bls = next_state.current_bls - tx.value;
        } else {
            next_state.next_bls = next_state.current_bls + tx.value;
        }

        tx.state_transition = Some(next_state.clone());

        return (tx, next_state.next_bls);
    }
}

impl FromRow<Transaction> for Transaction {
    fn from_row(row: Row<Complex>) -> anyhow::Result<Self> {
        let mut c = Self::default();

        c.cid = row.get("cid")?;
        c.height = row.get("height")?;
        c.block = row.get("block")?;
        c.message_rct_exit_code = row.get("message_rct_exit_code")?;
        c.message_rct_return = row.get("message_rct_return")?;
        c.message_rct_gas_used = row.get("message_rct_gas_used")?;
        c.sub_call_of = row.get("sub_call_of")?;
        c.from = row.get("from")?;
        c.to = row.get("to")?;
        c.robust_from = row.get("robust_from")?;
        c.robust_to = row.get("robust_to")?;
        c.gas_limit = row.get("gas_limit")?;
        c.gas_fee_cap = row.get("gas_fee_cap")?;
        c.gas_premium = row.get("gas_premium")?;
        c.method = row.get("method")?;
        c.params = row.get("params")?;
        c.value = row.get("value")?;
        c.timestamp = row.get("timestamp")?;
        c.nonce = row.get("nonce")?;

        c.state_transition = None;

        Ok(c)
    }
}

impl ApiResource for Transaction {
    fn get_table() -> String {
        return "flow.messages".to_string();
    }

    fn default_order_by() -> String {
        return "BlockTimestamp".to_string();
    }

    fn default_search_by() -> String {
        return "".to_string();
    }

    fn match_order_by(order_by: String) -> String {
        match order_by.to_lowercase().as_str() {
            "height" => "height".to_string(),
            _ => "timestamp".to_string(),
        }
    }

    fn match_search_by(search: String) -> Vec<String> {
        match search.to_lowercase().as_str() {
            "contract" => vec![
                "from".to_string(),
                "to".to_string(),
                "robust_from".to_string(),
                "robust_to".to_string(),
            ],
            "subcalls" => vec!["sub_call_of".to_string()],
            "block" => vec!["block".to_string()],
            _ => vec!["cid".to_string()],
        }
    }
}

#[derive(Deserialize)]
pub struct DecodeParamsBody {
    pub to: String,
    pub method: i64,
    pub params: String,
}
