use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;
use clickhouse_rs::types::{Complex, Row};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractBls {
    pub balance: i64,
    pub contract_id: String,
    pub transaction_count: i64,
}

impl FromRow<ContractBls> for ContractBls {
    fn from_row(row: Row<Complex>) -> anyhow::Result<ContractBls> {
        let mut c = Self::default();
        c.balance = row.get("Balance")?;
        c.contract_id = row.get("ContractId")?;
        c.transaction_count = row.get("TransactionCount")?;

        Ok(c)
    }
}

impl ApiResource for ContractBls {
    fn get_table() -> String {
        return "flow.contracts_bls".to_string();
    }

    fn default_order_by() -> String {
        return "TransactionCount".to_string();
    }

    fn default_search_by() -> String {
        return "ContractId".to_string();
    }

    fn match_order_by(_order_by: String) -> String {
        "TransactionCount".to_string()
    }

    fn match_search_by(_search: String) -> Vec<String> {
        vec!["ContractId".to_string()]
    }
}
