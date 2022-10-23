use clickhouse_rs::types::{Complex, Row};

pub trait FromRow<T> {
    fn from_row(row: Row<Complex>) -> anyhow::Result<T>;
}
