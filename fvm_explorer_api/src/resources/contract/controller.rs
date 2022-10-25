use crate::resources::contract::service::list;
use crate::ResourceService;
use actix_web::web;

pub struct ContractController {}

impl ResourceService for ContractController {
    fn configure(cfg: &mut web::ServiceConfig) -> () {
        cfg.service(web::resource("/contracts").route(web::get().to(list)));
    }
}
