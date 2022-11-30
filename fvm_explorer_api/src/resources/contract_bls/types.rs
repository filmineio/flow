use clickhouse_rs::types::{Complex, Row};
use serde::{Deserialize, Serialize};

use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractBls {
    pub contract_id: String,
    pub ok_transaction_count: u64,
    pub reverted_transaction_count: u64,
}

impl FromRow<ContractBls> for ContractBls {
    fn from_row(row: Row<Complex>) -> anyhow::Result<ContractBls> {
        let mut c = Self::default();
        c.contract_id = row.get("ContractAddress")?;
        c.ok_transaction_count = row.get("TransactionCountOk")?;
        c.reverted_transaction_count = row.get("TransactionCountReverted")?;

        Ok(c)
    }
}

impl ApiResource for ContractBls {
    fn get_table() -> String {
        "flow.contracts".to_string()
    }

    fn default_order_by() -> String {
        "ContractAddress".to_string()
    }

    fn default_search_by() -> String {
        "ContractAddress".to_string()
    }

    fn match_order_by(_order_by: String) -> String {
        "ContractAddress".to_string()
    }

    fn match_search_by(_search: String) -> Vec<String> {
        vec!["ContractAddress".to_string()]
    }
}
