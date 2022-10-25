use crate::resources::user::service::{create, one};
use crate::shared::traits::resource_service::ResourceService;
use actix_web::web;

pub struct UserController {}

impl ResourceService for UserController {
    fn configure(cfg: &mut web::ServiceConfig) -> () {
        cfg.service(
            web::resource("/user")
                .route(web::post().to(create))
                .route(web::get().to(one)),
        );
    }
}
