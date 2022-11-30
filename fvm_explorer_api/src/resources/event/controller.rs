use actix_web::web;

use crate::ResourceService;

use super::service::list;

pub struct EventController {}

impl ResourceService for EventController {
    fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/events").route(web::get().to(list)));
    }
}
