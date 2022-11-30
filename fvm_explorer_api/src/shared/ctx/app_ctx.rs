use deadpool_postgres::Pool;
use lotus_rs::client::LotusClient;
use lotus_rs::config::LotusConfig;
use tokio_postgres::NoTls;

use crate::AppConfig;
use crate::shared::ctx::ch::CH;

#[derive(Clone)]
pub struct AppCtx {
    pub pg_pool: Pool,
    pub ch_pool: CH,
    pub lotus_client: LotusClient,
}

impl TryFrom<AppConfig> for AppCtx {
    type Error = anyhow::Error;

    fn try_from(value: AppConfig) -> Result<Self, Self::Error> {
        let ctx = AppCtx {
            pg_pool: value.pg.get_pool().create_pool(None, NoTls)?,
            ch_pool: CH::from(value),
            lotus_client: LotusClient::init(LotusConfig::from_env()),
        };
        Ok(ctx)
    }
}
