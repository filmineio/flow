use crate::types::state::execution_trace::InvocResult;
use crate::CID;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct State {
    pub Root: CID,
    pub Trace: Vec<InvocResult>,
}
