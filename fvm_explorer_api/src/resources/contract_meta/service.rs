use actix_web::{web, HttpResponse, Responder};



use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::api_helpers::api_response::to_res;


use crate::AppCtx;

use super::model::ContractMeta;
use super::types::ContractMetaPath;

pub async fn read(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    HttpResponse::Ok().json(to_res::<ContractMeta>(
        ContractMeta::read(&ctx.pg_pool, query.into_inner()).await,
        false,
    ))
}

pub async fn create(_data: web::Path<ContractMetaPath>, _ctx: web::Data<AppCtx>) -> impl Responder {
    // HttpResponse::Ok().json(to_res::<ContractMeta>(
    //     ContractMeta::create(&ctx.pg_pool, data.into_inner()).await,
    //     true,
    // ))

    HttpResponse::Ok().body("")
}
