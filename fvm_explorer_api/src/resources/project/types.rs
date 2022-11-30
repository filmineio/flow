use serde::{Deserialize, Serialize};

use crate::resources::contract_bls::types::ContractBls;
use crate::resources::project::model::Project;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectBody {
    pub owner_email: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectName {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPath {
    pub id: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOrRemoveContract {
    pub contract_id: String,
}

#[derive(Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullProject {
    pub id: i64,
    pub owner_email: String,
    pub name: String,
    pub contracts: Vec<ProjectContract>,
}

impl From<Project> for FullProject {
    fn from(value: Project) -> Self {
        Self {
            id: value.id,
            owner_email: value.owner_email,
            name: value.name,
            contracts: value
                .contracts
                .iter()
                .map(|v| ProjectContract::from(v.clone()))
                .collect::<Vec<ProjectContract>>(),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectContract {
    pub contract_id: String,
    pub ok_transaction_count: u64,
    pub reverted_transaction_count: u64,
}

impl From<String> for ProjectContract {
    fn from(value: String) -> Self {
        Self {
            contract_id: value,
            ok_transaction_count: 0,
            reverted_transaction_count: 0,
        }
    }
}

impl ProjectContract {
    pub fn set_bls(&mut self, bls: Vec<ContractBls>) {
        let binding = ContractBls::default();

        let bls = bls
            .iter()
            .find(|v| v.contract_id == self.contract_id)
            .unwrap_or(&binding);

        self.ok_transaction_count = bls.ok_transaction_count;
        self.reverted_transaction_count = bls.reverted_transaction_count;
    }
}
