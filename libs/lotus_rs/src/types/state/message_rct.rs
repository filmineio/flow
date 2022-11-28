use crate::CID;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageRct {
    pub ExitCode: Option<i8>,
    pub Return: Option<String>,
    pub GasUsed: Option<i64>,
    pub EventsRoot: Option<CID>,
}
