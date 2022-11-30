use deadpool_postgres::Pool;
use lotus_rs::client::LotusClient;
use lotus_rs::config::LotusConfig;
use tokio_postgres::NoTls;

use crate::shared::ctx::ch::CH;
use crate::shared::ctx::web3_storage::Web3Storage;
use crate::AppConfig;

#[derive(Clone)]
pub struct AppCtx {
    pub pg_pool: Pool,
    pub ch_pool: CH,
    pub lotus_client: LotusClient,
    pub w3s: Web3Storage,
}

impl TryFrom<AppConfig> for AppCtx {
    type Error = anyhow::Error;

    fn try_from(value: AppConfig) -> Result<Self, Self::Error> {
        let ctx = AppCtx {
            pg_pool: value.pg.get_pool().create_pool(None, NoTls)?,
            w3s: Web3Storage::from(&value),
            ch_pool: CH::from(value),
            lotus_client: LotusClient::init(LotusConfig::from_env()),
        };
        Ok(ctx)
    }
}
