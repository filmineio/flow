use super::service::list;
use crate::ResourceService;
use actix_web::web;

pub struct EventController {}

impl ResourceService for EventController {
    fn configure(cfg: &mut web::ServiceConfig) -> () {
        cfg.service(web::resource("/events").route(web::get().to(list)));
    }
}
