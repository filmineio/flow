use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;
use clickhouse_rs::types::{Complex, Row};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectContract {
    pub contract_id: String,
    pub ok_transaction_count: u64,
    pub reverted_transaction_count: u64,
}

impl FromRow<ProjectContract> for ProjectContract {
    fn from_row(row: Row<Complex>) -> anyhow::Result<ProjectContract> {
        let mut c = Self::default();
        c.contract_id = row.get("ContractAddress")?;
        c.ok_transaction_count = row.get("TransactionCountOk")?;
        c.reverted_transaction_count = row.get("TransactionCountReverted")?;

        Ok(c)
    }
}

impl ApiResource for ProjectContract {
    fn get_table() -> String {
        return "flow.contracts".to_string();
    }

    fn default_order_by() -> String {
        return "ContractAddress".to_string();
    }

    fn default_search_by() -> String {
        return "ContractAddress".to_string();
    }

    fn match_order_by(_order_by: String) -> String {
        "ContractAddress".to_string()
    }

    fn match_search_by(_search: String) -> Vec<String> {
        vec!["ContractAddress".to_string()]
    }
}
