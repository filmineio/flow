use serde::{Deserialize, Serialize};

use crate::shared::traits::api_resource::ApiResource;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum SortDirection {
    ASC,
    DESC,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ApiQuery {
    pub search: Option<String>,
    pub search_by: Option<String>,
    pub limit: Option<i64>,
    pub skip: Option<i64>,
    pub order_by: Option<String>,
    pub order_direction: Option<SortDirection>,
}

impl ApiQuery {
    pub fn get_sort_direction(&self) -> String {
        let v = self.order_direction.clone().unwrap_or(SortDirection::ASC);

        match v {
            SortDirection::ASC => "ASC".to_string(),
            SortDirection::DESC => "DESC".to_string(),
        }
    }

    pub fn get_order_by<T: ApiResource>(&self) -> String {
        T::match_order_by(self.order_by.clone())
    }

    pub fn get_search_by<T: ApiResource>(&self) -> Vec<String> {
        T::match_search_by(self.search_by.clone())
    }

    pub fn get_search_term(&self) -> Option<String> {
        self.search
            .as_ref()
            .map(|val| sql_lexer::sanitize_string(val.to_string()))
    }
}
