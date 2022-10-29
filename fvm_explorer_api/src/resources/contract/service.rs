use super::types::{Contract, ContractBytecode, ContractBytecodePath};
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::utils::query_utils::QueryUtils;
use crate::AppCtx;
use actix_web::{web, HttpResponse, Responder};

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    if let Some(res) = ctx
        .ch_pool
        .query::<Contract>(&format!(
            "{} left join (select MessageRctExitCode, From, count(MessageRctExitCode) as C from flow.messages where MessageRctExitCode = 0 GROUP BY (From, MessageRctExitCode)) t_ok_o on t_ok_o.From = ContractId left join (select MessageRctExitCode, To, count(MessageRctExitCode) as C from flow.messages where MessageRctExitCode = 0 GROUP BY (To, MessageRctExitCode)) t_ok_in on t_ok_in.To = ContractId left join (select MessageRctExitCode, From, count(MessageRctExitCode) as C from flow.messages where MessageRctExitCode != 0 GROUP BY (From, MessageRctExitCode)) t_rv_o on t_rv_o.From = ContractId left join (select MessageRctExitCode, To, count(MessageRctExitCode) as C from flow.messages where MessageRctExitCode != 0 GROUP BY (To, MessageRctExitCode)) t_rv_in on t_rv_in.To = ContractId {}",
            QueryUtils::prepare_query::<Contract>(vec![
                        "OwnerAddress",
        "ContractAddress",
        "OwnerId",
        "ContractId",
        "Compiler",
        "ContractType",
        "EthAddress",
        "ContractActorAddress",
                "plus(t_ok_o.C, t_ok_in.C) as TransactionCountOk", 
                "plus(t_rv_o.C, t_rv_in.C) as TransactionCountReverted"
            ]),
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
