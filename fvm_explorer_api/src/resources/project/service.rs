use actix_web::{web, HttpResponse, Responder};

use serde_json::Value::Null;


use crate::resources::contract_bls::types::ContractBls;
use crate::resources::project::model::Project;
use crate::resources::project::types::{
    AddOrRemoveContract, CreateProjectBody, FullProject, ProjectContract, ProjectPath,
    UpdateProjectName,
};
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::api_helpers::api_response::to_res;
use crate::shared::types::result_with_total::ResultWithTotal;
use crate::shared::utils::query_utils::{QueryUtils};
use crate::AppCtx;

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    let data = to_res::<Project>(
        Project::find_all(&ctx.pg_pool, query.into_inner()).await,
        false,
    );
    let mut bls: Vec<ContractBls> = vec![];

    if !data.rows.is_empty() {
        let mut contracts = vec![];
        data.rows.iter().for_each(|v| {
            if !v.contracts.is_empty() {
                v.contracts.iter().for_each(|c| {
                    if !contracts.contains(c) {
                        contracts.push(c.clone())
                    }
                })
            }
        });

        if let Some(res) = ctx
            .ch_pool
            .query::<ContractBls>(&format!(
                "{} left join (select MessageRctExitCode, From, count(MessageRctExitCode) as C from flow.messages where MessageRctExitCode = 0 GROUP BY (From, MessageRctExitCode)) t_ok_o on t_ok_o.From = ContractId left join (select MessageRctExitCode, To, count(MessageRctExitCode) as C from flow.messages where MessageRctExitCode = 0 GROUP BY (To, MessageRctExitCode)) t_ok_in on t_ok_in.To = ContractId left join (select MessageRctExitCode, From, count(MessageRctExitCode) as C from flow.messages where MessageRctExitCode != 0 GROUP BY (From, MessageRctExitCode)) t_rv_o on t_rv_o.From = ContractId left join (select MessageRctExitCode, To, count(MessageRctExitCode) as C from flow.messages where MessageRctExitCode != 0 GROUP BY (To, MessageRctExitCode)) t_rv_in on t_rv_in.To = ContractId  WHERE ContractAddress IN ({}) ",
                QueryUtils::prepare_query::<ContractBls>(vec![
                    "ContractAddress",
                    "plus(t_ok_o.C, t_ok_in.C) as TransactionCountOk",
                    "plus(t_rv_o.C, t_rv_in.C) as TransactionCountReverted"
                ]),
                contracts
                    .clone()
                    .iter()
                    .map(|v| format!("'{}'", v))
                    .collect::<Vec<String>>()
                    .join(",")
            ))
            .await
        {
            bls = res.rows;
        }
    }

    let mut res = ResultWithTotal::default();
    res.total = data.total;
    res.rows = data
        .rows
        .iter()
        .map(|r| {
            let mut v = FullProject::from(r.clone());
            v.contracts = v
                .contracts
                .iter()
                .map(|c| {
                    let mut v = ProjectContract::from(c.contract_id.clone());
                    v.set_bls(bls.clone());
                    v
                })
                .collect::<Vec<ProjectContract>>();
            v
        })
        .collect();

    HttpResponse::Ok().json(res)
}

pub async fn create(data: web::Json<CreateProjectBody>, ctx: web::Data<AppCtx>) -> impl Responder {
    HttpResponse::Ok().json(to_res::<Project>(
        Project::create(&ctx.pg_pool, data.into_inner()).await,
        true,
    ))
}

pub async fn update(
    data: web::Json<UpdateProjectName>,
    path: web::Path<ProjectPath>,
    ctx: web::Data<AppCtx>,
) -> impl Responder {
    HttpResponse::Ok().json(to_res::<Project>(
        Project::update_name(&ctx.pg_pool, path.into_inner().id, data.into_inner().name).await,
        true,
    ))
}

pub async fn add_contract(
    data: web::Json<AddOrRemoveContract>,
    info: web::Path<ProjectPath>,
    ctx: web::Data<AppCtx>,
) -> impl Responder {
    HttpResponse::Ok().json(to_res::<Project>(
        Project::toggle_contract(
            &ctx.pg_pool,
            info.into_inner().id,
            data.into_inner().contract_id,
            true,
        )
        .await,
        true,
    ))
}

pub async fn remove_contract(
    data: web::Json<AddOrRemoveContract>,
    info: web::Path<ProjectPath>,
    ctx: web::Data<AppCtx>,
) -> impl Responder {
    HttpResponse::Ok().json(to_res::<Project>(
        Project::toggle_contract(
            &ctx.pg_pool,
            info.into_inner().id,
            data.into_inner().contract_id,
            false,
        )
        .await,
        true,
    ))
}

pub async fn delete(info: web::Path<ProjectPath>, ctx: web::Data<AppCtx>) -> impl Responder {
    let res = Project::delete(&ctx.pg_pool, info.into_inner().id).await;

    match res {
        Ok(_) => HttpResponse::Ok().json(ResultWithTotal::<()>::default()),
        Err(_) => HttpResponse::BadRequest().json(Null),
    }
}
