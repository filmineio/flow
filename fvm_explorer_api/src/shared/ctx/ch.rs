use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;
use crate::shared::types::result_with_total::ResultWithTotal;
use crate::shared::utils::query_utils::TOTAL_RES_KEY;
use crate::AppConfig;
use anyhow::anyhow;
use clickhouse_rs::Pool;
use log::error;

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
    pub async fn insert(&self, table: String, block: clickhouse_rs::Block) -> anyhow::Result<()> {
        match self.pool.get_handle().await {
            Ok(mut client) => match client.insert(table, block).await {
                Ok(_) => Ok(()),
                Err(e) => Err(anyhow!("Failed to insert {:?}", e)),
            },
            Err(e) => Err(anyhow!("Failed to connect {:?}", e)),
        }
    }

    pub async fn query<DT: FromRow<DT> + ApiResource + Default + Clone>(
        &self,
        query: &String,
    ) -> Option<ResultWithTotal<DT>> {
        match self.pool.get_handle().await {
            Ok(mut client) => match client.query(query).fetch_all().await {
                Ok(res) => {
                    let mut r = ResultWithTotal::default();

                    let rows = res
                        .rows()
                        .map(|v| {
                            r.set_total_u(v.get(TOTAL_RES_KEY).unwrap_or(0));
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
}
