use super::types::{DecodeParamsBody, Transaction};
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::utils::query_utils::QueryUtils;
use crate::AppCtx;
use actix_web::{web, HttpResponse, Responder};
use serde_json::Value::Null;

pub async fn list(q: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    let mut query = q.into_inner();
    let mut limit = query.limit.clone().unwrap_or(1);
    let mut skip = query.skip.clone().unwrap_or(1);

    if let Some(v) = query.search_by.clone() {
        if v == "contract" {
            query.limit = Some(2000);
            query.skip = Some(0);
        }
    }
    if let Some(mut res) = ctx
        .ch_pool
        .query::<Transaction>(&format!(
            "{} {}",
            QueryUtils::prepare_query::<Transaction>(vec!["*"]),
            QueryUtils::get_query_filters::<Transaction>(query.clone())
        ))
        .await
    {
        if let Some(v) = query.search_by.clone() {
            if v == "contract" {
                let mut current_bls = 0;
                res.rows = res
                    .rows
                    .iter()
                    .map(|r| {
                        let (tx, new_state) = Transaction::set_state_transition(
                            r.clone(),
                            query.search.clone().unwrap_or("".to_string()),
                            current_bls,
                        );

                        current_bls = new_state;
                        tx
                    })
                    .collect();

                let rows_len: i64 = res.rows.len().try_into().unwrap_or(0);

                skip = skip.min(rows_len);
                limit = limit.min(rows_len - skip);

                res.rows = res
                    .rows
                    .drain(skip.try_into().unwrap_or(0)..(skip + limit).try_into().unwrap_or(1))
                    .collect();
                return HttpResponse::Ok().json(res);
            }
        }

        return HttpResponse::Ok().json(res);
    }

    let default: Vec<Transaction> = vec![];
    HttpResponse::Ok().json(default)
}

pub async fn decode_params(
    info: web::Json<DecodeParamsBody>,
    ctx: web::Data<AppCtx>,
) -> impl Responder {
    if let Ok(res) = ctx
        .lotus_client
        .state_decode_params(info.to.clone(), info.method.clone(), info.params.clone())
        .await
    {
        return HttpResponse::Ok().json(res);
    }
    return HttpResponse::Ok().json(Null);
}
