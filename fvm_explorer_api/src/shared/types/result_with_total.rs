use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ResultWithTotal<T: FromRow<T> + ApiResource + Default + Clone> {
    pub total: u64,
    pub network: String,
    pub rows: Vec<T>,
}
