use actix_web::web;

use crate::ResourceService;

use super::service::{create, read};

pub struct ContractMetaController {}

impl ResourceService for ContractMetaController {
    fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/contract-meta/{id}")
                .route(web::get().to(read))
                .route(web::post().to(create)),
        );
    }
}
