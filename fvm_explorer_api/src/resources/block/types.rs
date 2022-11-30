use clickhouse_rs::types::{Complex, Row};
use serde::{Deserialize, Serialize};

use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    cid: String,
    block: String,
}

impl FromRow<Block> for Block {
    fn from_row(row: Row<Complex>) -> anyhow::Result<Self> {
        let mut c = Self::default();

        c.cid = row.get("Cid")?;
        c.block = row.get("Block")?;

        Ok(c)
    }
}

impl ApiResource for Block {
    fn get_table() -> String {
        "flow.block".to_string()
    }

    fn default_order_by() -> String {
        "Timestamp".to_string()
    }

    fn default_search_by() -> String {
        "".to_string()
    }

    fn match_order_by(order_by: String) -> String {
        match order_by.to_lowercase().as_str() {
            _ => "Cid".to_string(),
        }
    }

    fn match_search_by(search: String) -> Vec<String> {
        match search.to_lowercase().as_str() {
            _ => vec!["Cid".to_string()],
        }
    }
}
