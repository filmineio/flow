use anyhow::Result;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::resources::project::types::CreateProjectBody;
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::traits::api_resource::ApiResource;
use crate::shared::utils::query_utils::QueryUtils;

#[derive(Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i64,
    pub owner_email: String,
    pub name: String,
    pub contracts: Vec<String>,
}

impl From<Row> for Project {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            owner_email: value.get("owner_email"),
            name: value.get("name"),
            contracts: value.get("contracts"),
        }
    }
}
impl ApiResource for Project {
    fn get_table() -> String {
        "projects".to_string()
    }

    fn default_order_by() -> String {
        "id".to_string()
    }

    fn default_search_by() -> String {
        "id".to_string()
    }

    fn match_order_by(_order_by: String) -> String {
        "id".to_string()
    }

    fn match_search_by(search: String) -> Vec<String> {
        match search.as_str() {
            "name" => vec!["name".to_string()],
            "owner" => vec!["owner_email".to_string()],
            _ => vec!["id".to_string()],
        }
    }
}

impl Project {
    pub async fn find_all(db: &Pool, query: ApiQuery) -> Result<Vec<Row>> {
        let conn = db.get().await?;
        let projects = conn
            .query(
                format!(
                    "{} {}",
                    QueryUtils::prepare_query::<Project>(vec!["*"]),
                    QueryUtils::get_query_filters::<Project>(query)
                )
                .as_str(),
                &[],
            )
            .await?;
        Ok(projects)
    }
    pub async fn create(db: &Pool, project: CreateProjectBody) -> Result<Vec<Row>> {
        let conn = db.get().await?;
        let contracts: Vec<String> = vec![];
        let projects = conn
            .query(
                "INSERT INTO projects(owner_email, name, contracts) VALUES ($1, $2, $3) RETURNING *;",
                &[&project.owner_email, &project.name, &contracts],
            )
            .await?;

        Ok(projects)
    }
    pub async fn update_name(db: &Pool, id: i64, name: String) -> Result<Vec<Row>> {
        let conn = db.get().await?;

        let projects = conn
            .query(
                "UPDATE projects SET name=$1 WHERE id=$2 RETURNING *;",
                &[&name, &id],
            )
            .await?;

        Ok(projects)
    }

    pub async fn toggle_contract(
        db: &Pool,
        id: i64,
        contract: String,
        add: bool,
    ) -> Result<Vec<Row>> {
        let conn = db.get().await?;
        let mut operation =
            "UPDATE projects SET contracts = array_append(contracts, $1) WHERE id=$2 RETURNING *;";

        if !add {
            operation = "UPDATE projects SET contracts = array_remove(contracts, $1) WHERE id=$2 RETURNING *;";
        }

        let projects = conn.query(operation, &[&contract, &id]).await?;

        Ok(projects)
    }

    pub async fn delete(db: &Pool, id: i64) -> Result<()> {
        let conn = db.get().await?;

        let _projects = conn
            .query("DELETE FROM projects WHERE id={};", &[&id])
            .await?;

        Ok(())
    }
}
