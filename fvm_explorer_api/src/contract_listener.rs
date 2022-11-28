mod shared;

use crate::shared::app_config::app_config::AppConfig;
use crate::shared::ctx::app_ctx::AppCtx;
use crate::shared::listener::contract::Contract;
use crate::shared::listener::contract_transaction::ContractTransaction;
use crate::shared::listener::contract_type::ContractType;
use crate::shared::logger::logger::{Init, Logger};
use crate::shared::types::builtin_actors::eam::EAMReturn;
use anyhow::{anyhow, Result};
use fvm_ipld_encoding::RawBytes;
use fvm_shared::address::{Address, Network};
use kafka::consumer::Consumer;
use kafka::producer::AsBytes;
use log::{error, info, warn};
use serde_json::json;
use std::hash::Hash;
use std::time::Duration;
use tokio::time::sleep;
use tokio_postgres::types::IsNull::No;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    let config = AppConfig::init()?;
    Logger::init(config.server.logger_format);
    let ctx = AppCtx::try_from(config.clone())?;

    if let Err(e) = consume_messages(&ctx, &config).await {
        error!("{:?}", e);
    }

    Ok(())
}

fn test_address(addr: fvm_shared::address::Address) -> String {
    addr.to_string()
}

async fn consume_messages(ctx: &AppCtx, config: &AppConfig) -> Result<()> {
    let mut con = Consumer::from_hosts(vec![config.broker.connection_string.clone()])
        .with_topic(config.broker.new_contract_topic.clone())
        .create()?;

    loop {
        let mss = con.poll()?;
        if mss.is_empty() {
            println!("No messages available right now.");
            sleep(Duration::new(2, 0)).await
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                let transaction: ContractTransaction = serde_json::from_slice(m.value)?;
                if let Some(v) = transaction.message_rct_return.clone() {
                    if v.len() == 0 {
                        warn!(
                            "Ignoring Contract due to empty return {}",
                            transaction.cid.unwrap_or("".to_string())
                        );
                        continue;
                    }

                    let mut new_contract = Contract::try_from(transaction.clone())?;

                    match transaction.contract_type.clone() {
                        ContractType::EFVM => {
                            new_contract
                                .resolve_e_fvm_data(v, &ctx.lotus_client)
                                .await?;
                        }
                        ContractType::WASM => {
                            new_contract
                                .resolve_fvm_data(
                                    transaction.sub_call_of.unwrap_or("".to_string()),
                                    &ctx.lotus_client,
                                )
                                .await?
                        }
                    }

                    match ctx
                        .ch_pool
                        .insert("flow.contracts".to_string(), new_contract.get_ch_block())
                        .await
                    {
                        Ok(_) => info!("NEW CONTRACT CREATED: {:?}", new_contract),
                        Err(e) => error!("FAILED TO WRITE NEW CONTRACT {:?}", e),
                    }
                }
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}
