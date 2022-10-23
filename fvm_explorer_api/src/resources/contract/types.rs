use crate::shared::traits::from_ch_result::FromRow;
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
