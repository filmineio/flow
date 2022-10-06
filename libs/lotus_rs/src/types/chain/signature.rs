use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Signature {
    pub Data: String,
    pub Type: i32,
}
