use crate::shared::traits::from_ch_result::FromRow;
use crate::AppConfig;
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
    pub async fn query<DT: FromRow<DT> + Default + Clone>(&self, query: &str) -> Option<Vec<DT>> {
        match self.pool.get_handle().await {
            Ok(mut client) => match client.query(query).fetch_all().await {
                Ok(res) => Some(
                    res.rows()
                        .map(|v| DT::from_row(v).unwrap_or(DT::default()))
                        .collect::<Vec<DT>>(),
                ),
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
