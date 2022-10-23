use std::collections::HashMap;

pub type CID = HashMap<String, String>;

pub fn str2cid(str: String) -> CID {
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("/".to_string(), str);
    m
}

pub fn cid2str(cid: CID) -> Option<String> {
    let msg = cid.get("/");
    match msg {
        Some(c) => Some(c.clone()),
        None => None,
    }
}
