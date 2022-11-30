use actix_web::web;

use crate::resources::project::service::{add_contract, delete, remove_contract, update};
use crate::ResourceService;

use super::service::{create, list};

pub struct ProjectsController {}

impl ResourceService for ProjectsController {
    fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/projects")
                .route(web::get().to(list))
                .route(web::post().to(create)),
        );
        cfg.service(
            web::resource("/projects/{id}")
                .route(web::patch().to(update))
                .route(web::delete().to(delete)),
        );
        cfg.service(web::resource("/projects/{id}/add").route(web::patch().to(add_contract)));
        cfg.service(web::resource("/projects/{id}/remove").route(web::patch().to(remove_contract)));
    }
}
