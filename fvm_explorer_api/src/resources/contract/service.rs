use crate::resources::contract::types::Contract;
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::AppCtx;
use actix_web::{web, HttpResponse, Responder};

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    if let Some(res) = ctx
        .ch_pool
        .query::<Contract>(&format!(
            "{} {}",
            ctx.ch_pool.prepare_query::<Contract>(vec!["*"]),
            ctx.ch_pool.get_query_filters::<Contract>(query)
        ))
        .await
    {
        return HttpResponse::Ok().json(res);
    }

    let default: Vec<Contract> = vec![];
    HttpResponse::Ok().json(default)
}
