use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum SortDirection {
    ASC = 1,
    DESC = -1,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ApiQuery {
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub skip: Option<i64>,
    pub order_by: Option<String>,
    pub order_direction: Option<SortDirection>,
}

impl ApiQuery {
    pub fn get_pagination(&self) -> String {
        let mut str = String::from("");

        if let Some(order_by) = &self.order_by {
            str = format!(
                "{} ORDER BY {} {}",
                str,
                order_by,
                &self.get_sort_direction()
            )
        }

        if let Some(limit) = self.limit {
            str = format!("{} LIMIT {}", str, limit)
        }

        if let Some(skip) = self.skip {
            str = format!("{} OFFSET {}", str, skip)
        }

        str.clone()
    }

    fn get_sort_direction(&self) -> String {
        let v = self.order_direction.clone().unwrap_or(SortDirection::ASC);

        match v {
            SortDirection::ASC => "ASC".to_string(),
            SortDirection::DESC => "DESC".to_string(),
        }
    }
}
