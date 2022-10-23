use actix_web::web;

pub trait Resource {
    fn configure(cfg: &mut web::ServiceConfig) -> ();
}
