use crate::shared::traits::api_resource::ApiResource;
use crate::shared::traits::clickhouse::from_ch_result::FromRow;
use clickhouse_rs::types::{Complex, Row};
use lotus_rs::types::state::event::Entry;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub message_cid: String,
    pub events_root: String,
    pub emitter: i64,
    pub order: i64,
    pub entries: Vec<Entry>,
}

impl FromRow<Event> for Event {
    fn from_row(row: Row<Complex>) -> anyhow::Result<Self> {
        let mut c = Self::default();
        c.message_cid = row.get("MessageCid")?;
        c.events_root = row.get("EventsRoot")?;
        c.emitter = row.get("Emitter")?;
        c.order = row.get("Order")?;
        c.entries = match row.get("Entries") {
            Ok(v) => {
                let c: Vec<Entry> = serde_json::from_str(v)?;
                c
            }
            _ => {
                let c: Vec<Entry> = vec![];
                c
            }
        };

        Ok(c)
    }
}

impl ApiResource for Event {
    fn get_table() -> String {
        return "flow.events".to_string();
    }

    fn default_order_by() -> String {
        return "Order".to_string();
    }

    fn default_search_by() -> String {
        return "MessageCid".to_string();
    }

    fn match_order_by(order_by: String) -> String {
        match order_by.to_lowercase().as_str() {
            _ => "MessageCid".to_string(),
        }
    }

    fn match_search_by(search: String) -> Vec<String> {
        match search.to_lowercase().as_str() {
            _ => vec!["MessageCid".to_string(), "EventsRoot".to_string()],
        }
    }
}
