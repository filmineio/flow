use crate::AppCtx;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn create(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn one(_req: HttpRequest, _ctx: web::Data<AppCtx>) -> HttpResponse {
    HttpResponse::Ok().body("")
}
