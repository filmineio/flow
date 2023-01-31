use anyhow::anyhow;
use sled;

use crate::state_store::config::StateStoreConfig;
use crate::state_store::lib::SyncState;

#[derive(Debug)]
pub struct StateStore {
    db: sled::Db,
}

impl StateStore {
    pub fn new(config: StateStoreConfig) -> anyhow::Result<StateStore> {
        match sled::open(config.path.as_str()) {
            Ok(db) => Ok(StateStore { db }),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub async fn get_sync_state(&self) -> Option<SyncState> {
        match self.db.get(&"sync_state") {
            Ok(vec) => match vec {
                Some(v) => Some(bincode::deserialize(&v).unwrap()),
                _ => None,
            },
            _ => None,
        }
    }

    pub async fn update_sync_state(&self, sync_state: SyncState) -> anyhow::Result<()> {
        match bincode::serialize(&sync_state) {
            Ok(sync_state) => match self.db.insert(&"sync_state", sync_state) {
                Ok(_) => Ok(()),
                Err(e) => Err(anyhow!(e)),
            },
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub async fn get_current_height(&self) -> i64 {
        match self.get_sync_state().await {
            Some(state) => state.height,
            _ => 0,
        }
    }

    pub async fn update_current_height(&self, height: i64) -> anyhow::Result<()> {
        let mut sync_state = match self.get_sync_state().await {
            Some(s) => s,
            _ => SyncState {
                height: 0,
                block_cid: None,
                message_cid: None,
            },
        };

        sync_state.height = height;
        self.update_sync_state(sync_state).await
    }
}
