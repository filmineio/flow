use actix_web::web;

use crate::ResourceService;

use super::service::{decode_params, list};

pub struct TransactionController {}

impl ResourceService for TransactionController {
    fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/transactions").route(web::get().to(list)));
        cfg.service(
            web::resource("/transactions/decode-params").route(web::post().to(decode_params)),
        );
    }
}
