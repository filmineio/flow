pub trait ApiResource {
    fn get_table() -> String;
    fn default_order_by() -> String;
    fn default_search_by() -> String;
    fn match_order_by(order_by: String) -> String;
    fn match_search_by(search: String) -> Vec<String>;
}
