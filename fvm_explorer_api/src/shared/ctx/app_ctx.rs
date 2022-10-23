use crate::shared::ctx::ch::CH;
use crate::AppConfig;
use deadpool_postgres::Pool;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct AppCtx {
    pub pg_pool: Pool,
    pub ch_pool: CH,
}

impl TryFrom<AppConfig> for AppCtx {
    type Error = anyhow::Error;

    fn try_from(value: AppConfig) -> Result<Self, Self::Error> {
        let ctx = AppCtx {
            pg_pool: value.pg.get_pool().create_pool(None, NoTls)?,
            ch_pool: CH::from(value),
        };
        Ok(ctx)
    }
}
