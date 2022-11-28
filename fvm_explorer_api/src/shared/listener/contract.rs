use crate::shared::listener::contract_transaction::ContractTransaction;
use crate::shared::listener::contract_type::ContractType;
use crate::shared::types::builtin_actors::eam::EAMReturn;
use fvm_shared::address::Network;
use lotus_rs::client::LotusClient;
use lotus_rs::types::chain::cid::str2cid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    cid: Option<String>,
    contract_id: Option<String>,
    contract_address: Option<String>,
    contract_actor_address: Option<String>,
    owner_id: Option<String>,
    owner_address: Option<String>,
    compiler: Option<String>,
    contract_type: ContractType,
    eth_address: Option<String>,
    bytecode: Option<String>,
}

impl TryFrom<ContractTransaction> for Contract {
    type Error = anyhow::Error;
    fn try_from(value: ContractTransaction) -> anyhow::Result<Self> {
        let mut c = Self {
            cid: value.cid,
            contract_id: None,
            contract_address: None,
            contract_actor_address: None,
            owner_id: None,
            owner_address: None,
            compiler: Some("unknown".to_string()),
            contract_type: value.contract_type,
            eth_address: None,
            bytecode: None,
        };

        match c.contract_type {
            ContractType::WASM => {
                c.contract_id = value.to;
                c.contract_address = value.robust_to.clone();
                c.contract_actor_address = value.robust_to
            }
            ContractType::EFVM => {
                c.owner_id = value.from;
                c.owner_address = value.robust_from;
                c.bytecode = value.params
            }
        }

        return Ok(c);
    }
}

impl Contract {
    pub async fn resolve_e_fvm_data(
        &mut self,
        init_contract_return: String,
        lotus_client: &LotusClient,
    ) -> anyhow::Result<()> {
        fvm_shared::address::set_current_network(Network::Testnet);
        let decoded = &*base64::decode(init_contract_return)?;
        let res: EAMReturn = serde_ipld_dagcbor::from_slice(decoded)?;
        let actor_id = fvm_shared::address::Address::new_id(res.actor_id);
        let actor_state = lotus_client
            .state_get_actor(actor_id.to_string(), None)
            .await?;

        self.contract_id = Some(actor_id.to_string());
        self.contract_address = Some(actor_state.Address);
        self.eth_address = Some(format!("0x{}", hex::encode(&*res.eth_address)));
        self.contract_actor_address = Some(res.robust_address.to_string());

        Ok(())
    }

    pub async fn resolve_fvm_data(
        &mut self,
        kick_starter_message_cid: String,
        lotus_client: &LotusClient,
    ) -> anyhow::Result<()> {
        let starter_message = lotus_client
            .chain_get_message(str2cid(kick_starter_message_cid))
            .await?;

        if starter_message.From.len() < 8 {
            self.owner_id = Some(starter_message.From.clone());
            let actor_state = lotus_client
                .state_get_actor(starter_message.From.clone(), None)
                .await?;
            self.owner_address = Some(actor_state.Address);
        } else {
            self.owner_address = Some(starter_message.From.clone());
            self.owner_id = Some(
                lotus_client
                    .state_lookup_id(starter_message.From.clone(), None)
                    .await?,
            );
            self.bytecode = starter_message.Params
        }

        Ok(())
    }

    pub fn get_ch_block(&self) -> clickhouse_rs::Block {
        clickhouse_rs::Block::new()
            .column("Cid", vec![self.cid.clone().unwrap_or("".to_string())])
            .column(
                "ContractId",
                vec![self.contract_id.clone().unwrap_or("".to_string())],
            )
            .column(
                "ContractAddress",
                vec![self.contract_address.clone().unwrap_or("".to_string())],
            )
            .column(
                "ContractActorAddress",
                vec![self
                    .contract_actor_address
                    .clone()
                    .unwrap_or("".to_string())],
            )
            .column(
                "OwnerId",
                vec![self.owner_id.clone().unwrap_or("".to_string())],
            )
            .column(
                "OwnerAddress",
                vec![self.owner_address.clone().unwrap_or("".to_string())],
            )
            .column(
                "Bytecode",
                vec![self.bytecode.clone().unwrap_or("".to_string())],
            )
            .column(
                "Compiler",
                vec![self.compiler.clone().unwrap_or("unkown".to_string())],
            )
            .column("ContractType", vec![format!("{}", self.contract_type)])
            .column(
                "EthAddress",
                vec![self.eth_address.clone().unwrap_or("".to_string())],
            )
    }
}
