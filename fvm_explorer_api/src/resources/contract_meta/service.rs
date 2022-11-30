use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;

use super::model::ContractMeta;
use super::types::ContractMetaPath;
use crate::resources::contract::service::read_bytecode;
use crate::resources::contract_meta::types::ContractMetadata;
use crate::shared::api_helpers::api_query::ApiQuery;
use crate::shared::api_helpers::api_response::to_res;
use crate::shared::files::save::save_file;
use crate::shared::types::result_with_total::ResultWithTotal;
use crate::AppCtx;
use actix_multipart::Multipart;

pub async fn read(query: web::Query<ApiQuery>, ctx: web::Data<AppCtx>) -> impl Responder {
    HttpResponse::Ok().json(to_res::<ContractMeta>(
        ContractMeta::read(&ctx.pg_pool, query.into_inner()).await,
        false,
    ))
}

pub async fn create(
    data: web::Path<ContractMetaPath>,
    payload: Multipart,
    ctx: web::Data<AppCtx>,
) -> impl Responder {
    let v = data.into_inner();
    let fs = save_file(payload, v.contract_address.clone())
        .await
        .unwrap();
    // Take only first file until we resolve entry
    let contract_file = fs.get(0).unwrap();
    let output_dir = format!("output/{}", v.contract_address);

    let mut default: ResultWithTotal<ContractMetadata> = ResultWithTotal::default();

    let bytecode = read_bytecode(v.contract_address.clone(), &ctx).await;

    if bytecode.is_none() {
        return HttpResponse::Ok().json(default);
    }

    std::process::Command::new("solc")
        .arg("--bin")
        .arg("--hashes")
        .arg("--abi")
        .arg(contract_file)
        .arg("-o")
        .arg(&output_dir)
        .output()
        .unwrap();

    let filename = contract_file.split("/").last().unwrap();
    let contract_abi = format!("{}/{}", output_dir, filename.replace(".sol", ".abi"));

    let mut hm: HashMap<String, String> = HashMap::new();
    let cid = ctx.w3s.upload(contract_file.clone()).await.unwrap();
    hm.insert(filename.to_string(), cid);
    let mut paths = std::fs::read_dir(&output_dir).unwrap();

    let bin_path = paths
        .find(|path| {
            if let Ok(f) = path {
                return f.path().to_str().unwrap().to_string().contains(".bin");
            }

            return false;
        })
        .unwrap();

    let contents = std::fs::read_to_string(bin_path.unwrap().path())
        .expect("Should have been able to read the file");

    let bytecode = bytecode.unwrap();
    if contents.len() != bytecode.len() {
        println!("BYTECODES DO NOT MATCH {}, {}", contents, bytecode);
        return HttpResponse::Ok().json(default);
    }

    let mut paths = std::fs::read_dir(&output_dir).unwrap();
    for path in paths {
        if let Ok(f) = path {
            let cid = ctx
                .w3s
                .upload(f.path().to_str().unwrap().to_string())
                .await
                .unwrap();
            hm.insert(f.file_name().to_str().unwrap().to_string(), cid);
        }
    }

    let mut cc = ContractMetadata::default();

    let contents =
        std::fs::read_to_string(contract_abi).expect("Should have been able to read the file");
    let _c = ethabi::Contract::load(contents.as_bytes()).unwrap();

    for (k, v) in hm.iter() {
        if k.contains(".abi") {
            cc.abi_cid = v.clone();
        }

        if k.contains(".sol") {
            cc.main_cid = v.clone()
        }

        if k.contains(".bin") {
            cc.bin_cid = v.clone()
        }

        if k.contains(".signatures") {
            cc.sig_cid = v.clone()
        }
    }
    cc.contract_address = v.contract_address;
    cc.file_map = hm.clone();
    cc.compiler_version = "solidity >=0.4.22 <= 0.8.18".to_string();
    cc.name = filename.replace(".sol", "");

    // HttpResponse::Ok().json(to_res::<ContractMeta>(
    //     ContractMeta::create(&ctx.pg_pool, data.into_inner()).await,
    //     true,
    // ))

    default.total = 1;
    default.rows = vec![cc];
    HttpResponse::Ok().json(default)
}
