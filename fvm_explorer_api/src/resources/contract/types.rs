use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;
use clickhouse_rs::types::{Complex, Row};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    contract_id: String,
    contract_address: String,
    owner_id: String,
    owner_address: String,
    transaction_count: i64,
    balance: i64,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractBytecode {
    bytecode: String,
}

impl FromRow<ContractBytecode> for ContractBytecode {
    fn from_row(row: Row<Complex>) -> anyhow::Result<ContractBytecode> {
        let mut c = Self::default();
        c.bytecode = row.get("Bytecode")?;

        Ok(c)
    }
}

impl FromRow<Contract> for Contract {
    fn from_row(row: Row<Complex>) -> anyhow::Result<Self> {
        let mut c = Self::default();

        c.owner_address = row.get("OwnerRobustAddress")?;
        c.contract_address = row.get("ContractRobustAddress")?;
        c.owner_id = row.get("OwnerId")?;
        c.balance = row.get("Balance")?;
        c.transaction_count = row.get("TransactionCount")?;
        c.contract_id = row.get("ContractId")?;

        Ok(c)
    }
}

impl ApiResource for ContractBytecode {
    fn get_table() -> String {
        return "flow.contracts".to_string();
    }

    fn default_order_by() -> String {
        return "".to_string();
    }

    fn default_search_by() -> String {
        return "".to_string();
    }

    fn match_order_by(_order_by: String) -> String {
        "".to_string()
    }

    fn match_search_by(_search: String) -> Vec<String> {
        vec![
            "ContractId".to_string(),
            "ContractRobustAddress".to_string(),
        ]
    }
}

impl ApiResource for Contract {
    fn get_table() -> String {
        return "flow.contracts".to_string();
    }

    fn default_order_by() -> String {
        return "ContractId".to_string();
    }

    fn default_search_by() -> String {
        return "".to_string();
    }

    fn match_order_by(order_by: String) -> String {
        match order_by.to_lowercase().as_str() {
            "balance" => "Balance".to_string(),
            "transactioncount" => "TransactionCount".to_string(),
            _ => "ContractId".to_string(),
        }
    }

    fn match_search_by(search: String) -> Vec<String> {
        match search.to_lowercase().as_str() {
            "owner" => vec!["OwnerId".to_string(), "OwnerRobustAddress".to_string()],
            _ => vec![
                "ContractId".to_string(),
                "ContractRobustAddress".to_string(),
            ],
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ContractBytecodePath {
    pub contract_address: String,
}