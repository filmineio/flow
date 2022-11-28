use super::types::Event;
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::utils::query_utils::QueryUtils;
use crate::AppCtx;
use actix_web::{web, HttpResponse, Responder};

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    if let Some(res) = ctx
        .ch_pool
        .query::<Event>(&format!(
            "{} {}",
            QueryUtils::prepare_query::<Event>(vec!["*"]),
            QueryUtils::get_query_filters::<Event>(query.into_inner())
        ))
        .await
    {
        return HttpResponse::Ok().json(res);
    }

    let default: Vec<Event> = vec![];
    HttpResponse::Ok().json(default)
}
