use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;
use crate::shared::types::result_with_total::ResultWithTotal;
use crate::AppConfig;
use actix_web::web;
use clickhouse_rs::Pool;
use log::error;

const TOTAL_RES_KEY: &str = "__Total";
const TOTAL_RES: &str = "COUNT(*) OVER() as __Total";

#[derive(Clone)]
pub struct CH {
    pool: Pool,
}

impl From<AppConfig> for CH {
    fn from(value: AppConfig) -> Self {
        CH {
            pool: Pool::new(value.ch.connection_string),
        }
    }
}

impl CH {
    pub fn prepare_query<DT: FromRow<DT> + ApiResource + Default + Clone>(
        &self,
        fields: Vec<&str>,
    ) -> String {
        format!(
            "SELECT {}, {}  FROM {} ",
            TOTAL_RES,
            fields.join(","),
            DT::get_table()
        )
    }

    pub async fn query<DT: FromRow<DT> + ApiResource + Default + Clone>(
        &self,
        query: &str,
    ) -> Option<ResultWithTotal<DT>> {
        match self.pool.get_handle().await {
            Ok(mut client) => match client.query(query).fetch_all().await {
                Ok(res) => {
                    let mut r = ResultWithTotal {
                        total: 0,
                        rows: vec![],
                    };

                    let rows = res
                        .rows()
                        .map(|v| {
                            r.total = v.get(TOTAL_RES_KEY).unwrap_or(0);
                            DT::from_row(v).unwrap_or(DT::default())
                        })
                        .collect::<Vec<DT>>();

                    r.rows = rows;
                    Some(r)
                }
                Err(e) => {
                    error!("Clickhouse Query Error: {:?}", e);
                    None
                }
            },
            Err(e) => {
                error!("Clickhouse Pool Error: {:?}", e);
                None
            }
        }
    }

    pub fn get_query_filters<DT: FromRow<DT> + ApiResource + Default + Clone>(
        &self,
        query: web::Query<ApiQuery>,
    ) -> String {
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
}
