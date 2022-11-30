
use serde::Serialize;
use tokio_postgres::Row;

use crate::shared::types::result_with_total::ResultWithTotal;
use crate::shared::utils::query_utils::TOTAL_RES_KEY;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: Serialize + Default> {
    total_results: i64,
    result: T,
}

pub fn to_res<T: Serialize + Default + Clone + From<Row>>(
    rows: anyhow::Result<Vec<Row>>,
    write_action: bool,
) -> ResultWithTotal<T> {
    match rows {
        Ok(v) => {
            let mut res = ResultWithTotal::default();

            for a in v {
                if write_action {
                    res.total = 1;
                } else {
                    res.total = a.get(TOTAL_RES_KEY);
                }
                res.rows.push(T::from(a));
            }

            res
        }
        Err(_e) => ResultWithTotal::<T>::default(),
    }
}
