use actix_web::{web, HttpResponse, Responder};
use ethabi::ethereum_types::Address;
use ethabi::ParamType::Bytes;
use ethabi::{Hash, Log, LogParam, ParamType, ParseLog, RawLog, Token};
use fvm_ipld_encoding::RawBytes;
use kafka::producer::AsBytes;
use lotus_rs::types::state::event::{Entry, Flags, StampedEvent};
use serde_ipld_dagcbor::DecodeError;
use serde_json::{Number, Value};
use std::convert::Infallible;

use crate::resources::event::decoder::decode_event;
use crate::resources::event::types::EthEvent;
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::types::result_with_total::ResultWithTotal;
use crate::shared::utils::query_utils::QueryUtils;
use crate::AppCtx;

use super::types::Event;

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    if let Some(mut res) = ctx
        .ch_pool
        .query::<Event>(&format!(
            "{} {}",
            QueryUtils::prepare_query::<Event>(vec!["*"]),
            QueryUtils::get_query_filters::<Event>(query.into_inner())
        ))
        .await
    {
        let mut result: ResultWithTotal<EthEvent> = ResultWithTotal::<EthEvent>::default();

        result.total = res.total;
        result.rows = res.rows.iter().map(|y| decode_event(y)).collect();

        return HttpResponse::Ok().json(result);
    }

    let default: ResultWithTotal<EthEvent> = ResultWithTotal::<EthEvent>::default();
    HttpResponse::Ok().json(default)
}
