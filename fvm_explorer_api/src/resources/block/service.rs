use super::types::Block;
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::utils::query_utils::QueryUtils;
use crate::AppCtx;
use actix_web::{web, HttpResponse, Responder};

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    if let Some(res) = ctx
        .ch_pool
        .query::<Block>(&format!(
                    "{} {}",
                    QueryUtils::prepare_query::<Block>(vec!["*"]),
                    QueryUtils::get_query_filters::<Block>(query.into_inner())
                ),
        )
        .await
    {
        return HttpResponse::Ok().json(res);
    }

    let default: Vec<Block> = vec![];
    HttpResponse::Ok().json(default)
}
