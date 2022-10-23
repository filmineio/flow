use crate::resources::contract::types::Contract;
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::AppCtx;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    if let Some(res) = ctx
        .ch_pool
        .query::<Contract>(&format!(
            "SELECT * FROM contracts {}",
            query.get_pagination()
        ))
        .await
    {
        return HttpResponse::Ok().json(res);
    }

    let default: Vec<Contract> = vec![];
    HttpResponse::Ok().json(default)
}

pub async fn one(_req: HttpRequest, _ctx: web::Data<AppCtx>) -> HttpResponse {
    HttpResponse::Ok().body("")
}
