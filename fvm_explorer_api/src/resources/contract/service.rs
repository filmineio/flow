use actix_web::{web, HttpResponse, Responder};
use kafka::producer::AsBytes;

use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::types::builtin_actors::eam::CreateParams;
use crate::shared::types::result_with_total::ResultWithTotal;
use crate::shared::utils::query_utils::QueryUtils;
use crate::AppCtx;

use super::types::{Contract, ContractBytecode, ContractBytecodePath};

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
    let mut default: ResultWithTotal<ContractBytecode> = ResultWithTotal::default();

    if let Some(bc) = read_bytecode(contract_path_info.into_inner().contract_address, &ctx).await {
        default.total = 1;
        default.rows = vec![ContractBytecode { bytecode: bc }];
    };
    HttpResponse::Ok().json(default)
}

pub async fn read_bytecode(address: String, ctx: &web::Data<AppCtx>) -> Option<String> {
    let mut query = ApiQuery::default();
    query.search = Some(address);
    query.limit = Some(1);
    query.skip = Some(0);

    if let Some(mut res) = ctx
        .ch_pool
        .query::<ContractBytecode>(&format!(
            "{} {}",
            QueryUtils::prepare_query::<ContractBytecode>(vec![
                "Bytecode",
                "ContractAddress",
                "ContractId",
                "EthAddress",
                "ContractActorAddress"
            ]),
            QueryUtils::get_query_filters::<ContractBytecode>(query)
        ))
        .await
    {
        if res.total == 0 {
            return None;
        }

        let bc = base64::decode(res.rows.last().unwrap().bytecode.clone()).unwrap();
        let contract: CreateParams = serde_ipld_dagcbor::from_slice(bc.as_bytes()).unwrap();
        let bc = hex::encode(contract.initcode.as_bytes());

        return Some(bc);
    }

    return None;
}
