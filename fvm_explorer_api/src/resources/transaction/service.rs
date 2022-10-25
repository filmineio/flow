use crate::resources::transaction::types::{DecodeParamsBody, StateTransition, Transaction};
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::AppCtx;
use actix_web::{web, HttpResponse, Responder};
use serde_json::Value::Null;

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    if let Some(mut res) = ctx
        .ch_pool
        .query::<Transaction>(&format!(
            "{} {}",
            ctx.ch_pool.prepare_query::<Transaction>(vec!["*"]),
            ctx.ch_pool.get_query_filters::<Transaction>(query.clone())
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
