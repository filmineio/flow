#![feature(box_into_inner)]

mod resources;
mod shared;
use crate::resources::user::controller::UserController;
use crate::shared::traits::resource_service::ResourceService;

use crate::resources::block::controller::BlockController;
use crate::resources::contract::controller::ContractController;
use crate::resources::transaction::controller::TransactionController;
use crate::shared::app_config::app_config::AppConfig;
use crate::shared::ctx::app_ctx::AppCtx;
use crate::shared::logger::logger::{Init, Logger};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    let config = AppConfig::init()?;
    Logger::init(config.server.logger_format);

    let port = config.server.port;
    let ctx = AppCtx::try_from(config)?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(ctx.clone()))
            .configure(UserController::configure)
            .configure(ContractController::configure)
            .configure(TransactionController::configure)
            .configure(BlockController::configure)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}
