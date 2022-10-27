use super::service::{create, list};
use crate::resources::project::service::{add_contract, remove_contract, update};
use crate::ResourceService;
use actix_web::web;

pub struct ProjectsController {}

impl ResourceService for ProjectsController {
    fn configure(cfg: &mut web::ServiceConfig) -> () {
        cfg.service(
            web::resource("/projects")
                .route(web::get().to(list))
                .route(web::post().to(create)),
        );
        cfg.service(web::resource("/projects/{id}").route(web::patch().to(update)));
        cfg.service(web::resource("/projects/{id}/add").route(web::patch().to(add_contract)));
        cfg.service(web::resource("/projects/{id}/remove").route(web::patch().to(remove_contract)));
    }
}
