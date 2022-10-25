use crate::shared::traits::api_resource::ApiResource;
use clickhouse_rs::types::{Complex, Row};

pub trait FromRow<T: ApiResource> {
    fn from_row(row: Row<Complex>) -> anyhow::Result<T>;
}
