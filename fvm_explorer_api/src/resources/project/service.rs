use crate::resources::contract_bls::types::ContractBls;
use crate::resources::project::model::Project;
use crate::resources::project::types::{
    AddOrRemoveContract, CreateProjectBody, FullProject, ProjectContract, ProjectPath,
    UpdateProjectName,
};
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::types::result_with_total::ResultWithTotal;
use crate::shared::utils::query_utils::{QueryUtils, TOTAL_RES_KEY};
use crate::AppCtx;
use actix_web::{web, HttpResponse, Responder};
use log::warn;
use serde_json::Value::Null;
use std::fmt::{format, Debug};
use tokio_postgres::Row;

pub async fn list(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    let mut data = to_res(
        Project::find_all(&ctx.pg_pool, query.into_inner()).await,
        false,
    );
    let mut bls: Vec<ContractBls> = vec![];

    if data.rows.len() > 0 {
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
                "{} WHERE ContractId IN ({}) ORDER BY TransactionCount DESC",
                QueryUtils::prepare_query::<ContractBls>(vec!["*"]),
                contracts
                    .clone()
                    .iter()
                    .map(|v| format!("'{}'", v).clone())
                    .collect::<Vec<String>>()
                    .join(",")
            ))
            .await
        {
            bls = res.rows.clone();
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
    HttpResponse::Ok().json(to_res(
        Project::create(&ctx.pg_pool, data.into_inner()).await,
        true,
    ))
}

pub async fn update(
    data: web::Json<UpdateProjectName>,
    path: web::Path<ProjectPath>,
    ctx: web::Data<AppCtx>,
) -> impl Responder {
    HttpResponse::Ok().json(to_res(
        Project::update_name(&ctx.pg_pool, path.into_inner().id, data.into_inner().name).await,
        true,
    ))
}

pub async fn add_contract(
    data: web::Json<AddOrRemoveContract>,
    info: web::Path<ProjectPath>,
    ctx: web::Data<AppCtx>,
) -> impl Responder {
    HttpResponse::Ok().json(to_res(
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
    HttpResponse::Ok().json(to_res(
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

fn to_res(rows: anyhow::Result<Vec<Row>>, write_action: bool) -> ResultWithTotal<Project> {
    match rows {
        Ok(v) => {
            let mut res = ResultWithTotal::default();

            let rows = v
                .iter()
                .map(|v| {
                    if write_action {
                        res.total = 1;
                    } else {
                        res.total = v.get(TOTAL_RES_KEY);
                    }
                    Project::try_from(v).unwrap()
                })
                .collect::<Vec<Project>>();

            res.rows = rows;
            res
        }
        Err(e) => {
            warn!("{:?}", e);
            ResultWithTotal::<Project>::default()
        }
    }
}
