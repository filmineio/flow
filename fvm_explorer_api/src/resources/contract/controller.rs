use actix_web::web;

use crate::ResourceService;

use super::service::{get_bytecode, list};

pub struct ContractController {}

impl ResourceService for ContractController {
    fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/contracts").route(web::get().to(list)));
        cfg.service(
            web::resource("/contracts/{contract_address}/bytecode")
                .route(web::get().to(get_bytecode)),
        );
    }
}
