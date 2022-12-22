use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;

use crate::resources::block::controller::BlockController;
use crate::resources::contract::controller::ContractController;
use crate::resources::contract_meta::controller::ContractMetaController;
use crate::resources::event::controller::EventController;
use crate::resources::project::controller::ProjectsController;
use crate::resources::transaction::controller::TransactionController;
use crate::resources::user::controller::UserController;
use crate::shared::app_config::app_config::AppConfig;
use crate::shared::ctx::app_ctx::AppCtx;
use crate::shared::logger::logger::{Init, Logger};
use crate::shared::traits::resource_service::ResourceService;

mod resources;
mod shared;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    let config = AppConfig::init()?;
    Logger::init(config.server.logger_format);

    let port = config.server.port;
    let ctx = AppCtx::try_from(config)?;

    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .app_data(Data::new(ctx.clone()))
            .configure(UserController::configure)
            .configure(ContractController::configure)
            .configure(TransactionController::configure)
            .configure(BlockController::configure)
            .configure(ProjectsController::configure)
            .configure(EventController::configure)
            .configure(ContractMetaController::configure)
            .wrap(cors)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}
