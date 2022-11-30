use crate::shared::app_config::app_config::AppConfig;
use cid::Cid;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Web3Storage {
    auth_token: String,
}

impl From<&AppConfig> for Web3Storage {
    fn from(v: &AppConfig) -> Self {
        Self {
            auth_token: v.w3s.token.clone(),
        }
    }
}

impl Web3Storage {
    pub async fn upload(&self, path: String) -> anyhow::Result<String> {
        let cid_result = w3s::helper::upload(
            &path,            // the file path
            &self.auth_token, // the api token created in web3.storage
            2,                // max concurrent upload threads
            Some(Arc::new(Mutex::new(|name, part, pos, total| {
                // the progress listener
                println!("name: {name} part:{part} {pos}/{total}");
            }))),
            Some(None), // if packed in CAR with custom block size, `Some(None)` means packed in CAR with default 256K block size
            None,
            Some(None), // if use compression with zstd level, `Some(None)` means uses compression with zstd level at 10
        )
        .await?;

        Ok(cid_result.first().unwrap().to_string())
    }
}
