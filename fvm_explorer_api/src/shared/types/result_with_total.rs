use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResultWithTotal<T: Default + Clone> {
    pub total: i64,
    pub network: String,
    pub rows: Vec<T>,
}

impl<T: Default + Clone> Default for ResultWithTotal<T> {
    fn default() -> Self {
        Self {
            total: 0,
            network: "Wallabynet".to_string(),
            rows: vec![],
        }
    }
}
impl<T: Default + Clone> ResultWithTotal<T> {
    pub fn set_total_u(&mut self, v: u64) {
        self.total = v as i64;
    }
}
