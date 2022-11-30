use actix_web::web;

pub trait ResourceService {
    fn configure(cfg: &mut web::ServiceConfig);
}
