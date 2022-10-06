use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Loc {
    pub File: String,
    pub Line: String,
    pub Function: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GasCharge {
    pub Name: String,
    pub tg: Option<i64>,
    pub cg: Option<i64>,
    pub sg: Option<i64>,
    pub vtg: Option<i64>,
    pub vcg: Option<i64>,
    pub vsg: Option<i64>,
    pub tt: Option<i64>,
    pub ex: Option<HashMap<String, i64>>,
}
