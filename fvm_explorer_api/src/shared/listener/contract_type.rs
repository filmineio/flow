use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ContractType {
    WASM,
    EFVM,
}

impl Display for ContractType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractType::WASM => {
                write!(f, "WASM")?;
            }
            ContractType::EFVM => {
                write!(f, "EFVM")?;
            }
        }
        Ok(())
    }
}
