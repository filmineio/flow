use super::types::{Contract, ContractBytecode, ContractBytecodePath};
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::utils::query_utils::QueryUtils;
use crate::AppCtx;
use actix_web::{web, HttpResponse, Responder};

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    if let Some(res) = ctx
        .ch_pool
        .query::<Contract>(&format!(
            "{} left join flow.contracts_bls b on ContractRobustAddress = b.ContractId {}",
            QueryUtils::prepare_query::<Contract>(vec!["*"]),
            QueryUtils::get_query_filters::<Contract>(query.into_inner())
        ))
        .await
    {
        return HttpResponse::Ok().json(res);
    }

    let default: Vec<Contract> = vec![];
    HttpResponse::Ok().json(default)
}

pub async fn get_bytecode(
    contract_path_info: web::Path<ContractBytecodePath>,
    ctx: web::Data<AppCtx>,
) -> impl Responder {
    let mut query = ApiQuery::default();
    query.search = Some(contract_path_info.into_inner().contract_address);

    if let Some(res) = ctx
        .ch_pool
        .query::<ContractBytecode>(&format!(
            "{} {}",
            QueryUtils::prepare_query::<ContractBytecode>(vec!["Bytecode"]),
            QueryUtils::get_query_filters::<ContractBytecode>(query)
        ))
        .await
    {
        return HttpResponse::Ok().json(res);
    }

    let default: Vec<Contract> = vec![];
    HttpResponse::Ok().json(default)
}
