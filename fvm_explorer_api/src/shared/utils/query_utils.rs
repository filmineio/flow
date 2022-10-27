use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::traits::api_resource::ApiResource;

pub struct QueryUtils {}

pub const TOTAL_RES_KEY: &str = "total";
const TOTAL_RES: &str = "COUNT(*) OVER() as total";

pub enum Match {
    Strict,
    In,
    ILike,
}

impl QueryUtils {
    pub fn get_query_filters<DT: ApiResource + Default + Clone>(query: ApiQuery) -> String {
        let mut query_string = "".to_string();

        if let Some(search) = query.get_search_term() {
            query_string = format!(
                "{} WHERE {}",
                query_string,
                query
                    .get_search_by::<DT>()
                    .iter()
                    .map(move |v| format!("{} = '{}'", v, search))
                    .collect::<Vec<String>>()
                    .join(" OR ")
            )
        }

        format!(
            "{} ORDER BY {} {} OFFSET {} ROWS FETCH NEXT {} ROWS ONLY",
            query_string,
            query.get_order_by::<DT>(),
            &query.get_sort_direction(),
            query.skip.clone().unwrap_or(0),
            query.limit.unwrap_or(1)
        )
    }

    pub fn prepare_query<DT: ApiResource + Default + Clone>(fields: Vec<&str>) -> String {
        format!(
            "SELECT {}, {}  FROM {} ",
            fields.join(","),
            TOTAL_RES,
            DT::get_table()
        )
    }
}
