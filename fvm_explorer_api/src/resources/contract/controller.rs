use crate::resources::contract::handlers::{list, one};
use crate::Resource;
use actix_web::web;

pub struct ContractController {}

impl Resource for ContractController {
    fn configure(cfg: &mut web::ServiceConfig) -> () {
        cfg.service(web::resource("/contracts").route(web::get().to(list)))
            .service(web::resource("/contracts/:address").route(web::get().to(one)));
    }
}
