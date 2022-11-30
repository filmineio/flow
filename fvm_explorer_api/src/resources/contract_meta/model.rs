use anyhow::Result;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_postgres::Row;

use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::traits::api_resource::ApiResource;
use crate::shared::utils::query_utils::QueryUtils;

use super::types::ContractMetadata;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractMeta {
    pub id: i64,
    pub contract_address: String,
    pub abi_cid: String,
    pub main_cid: String,
    pub sig_cid: String,
    pub bin_cid: String,
    pub name: String,
    pub compiler_version: String,
    // file_map is a cid -> file name map
    pub file_map: HashMap<String, String>,
}

impl From<Row> for ContractMeta {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            contract_address: value.get("contract_address"),
            abi_cid: value.get("abi_cid"),
            main_cid: value.get("main_cid"),
            sig_cid: value.get("sig_cid"),
            bin_cid: value.get("bin_cid"),
            name: value.get("name"),
            compiler_version: value.get("compiler_version"),
            file_map: serde_json::from_str(value.get("file_map")).unwrap(),
        }
    }
}
impl ApiResource for ContractMeta {
    fn get_table() -> String {
        "contract_meta".to_string()
    }

    fn default_order_by() -> String {
        "id".to_string()
    }

    fn default_search_by() -> String {
        "contract_address".to_string()
    }

    fn match_order_by(_order_by: String) -> String {
        "id".to_string()
    }

    fn match_search_by(search: String) -> Vec<String> {
        match search.as_str() {
            _ => vec!["contract_address".to_string()],
        }
    }
}

impl ContractMeta {
    pub async fn read(db: &Pool, query: ApiQuery) -> Result<Vec<Row>> {
        let conn = db.get().await?;
        let projects = conn
            .query(
                format!(
                    "{} {}",
                    QueryUtils::prepare_query::<ContractMeta>(vec!["*"]),
                    QueryUtils::get_query_filters::<ContractMeta>(query)
                )
                .as_str(),
                &[],
            )
            .await?;
        Ok(projects)
    }
    pub async fn create(db: &Pool, data: ContractMetadata) -> Result<Vec<Row>> {
        let conn = db.get().await?;
        let res = conn
            .query(
                "INSERT INTO contract_meta(\
                    contract_address,\
                    abi_cid,\
                    main_cid,\
                    name,\
                    compiler_version,\
                    file_map
                ) \
                VALUES (\
                    $1,\
                    $2,\
                    $3,\
                    $4,\
                    $5,\
                    $6\
                ) RETURNING *;",
                &[
                    &data.contract_address,
                    &data.abi_cid,
                    &data.main_cid,
                    &data.name,
                    &data.compiler_version,
                    &serde_json::to_string_pretty(&data.file_map)?,
                ],
            )
            .await?;

        Ok(res)
    }
}
