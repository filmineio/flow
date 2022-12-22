pub trait ApiResource {
    fn get_table() -> String;
    fn match_order_by(order_by: Option<String>) -> String;
    fn match_search_by(search: Option<String>) -> Vec<String>;
}
