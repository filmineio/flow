use crate::resources::block::service::list;
use crate::ResourceService;
use actix_web::web;

pub struct BlockController {}

impl ResourceService for BlockController {
    fn configure(cfg: &mut web::ServiceConfig) -> () {
        cfg.service(web::resource("/blocks").route(web::get().to(list)));
    }
}
