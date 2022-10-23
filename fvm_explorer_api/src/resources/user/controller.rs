use crate::resources::user::handlers::{create, one};
use crate::shared::traits::resource::Resource;
use actix_web::web;

pub struct UserController {}

impl Resource for UserController {
    fn configure(cfg: &mut web::ServiceConfig) -> () {
        cfg.service(
            web::resource("/user")
                .route(web::post().to(create))
                .route(web::get().to(one)),
        );
    }
}
