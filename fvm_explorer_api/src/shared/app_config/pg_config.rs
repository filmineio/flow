use serde::Deserialize;

use crate::shared::encoding::json::de_u16_from_str;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct PostgresConfig {
    #[serde(deserialize_with = "de_u16_from_str")]
    pub port: u16,
    pub user: String,
    pub password: String,
    pub host: String,
    pub dbname: String,
}

impl PostgresConfig {
    pub fn get_pool(&self) -> deadpool_postgres::Config {
        let mut cfg = deadpool_postgres::Config::default();
        cfg.user = Some(self.user.clone());
        cfg.port = Some(self.port);
        cfg.password = Some(self.password.clone());
        cfg.host = Some(self.host.clone());
        cfg.dbname = Some(self.dbname.clone());

        cfg
    }
}
