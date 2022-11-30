use clickhouse_rs::types::{Complex, Row};

use crate::shared::traits::api_resource::ApiResource;

pub trait FromRow<T: ApiResource> {
    fn from_row(row: Row<Complex>) -> anyhow::Result<T>;
}
