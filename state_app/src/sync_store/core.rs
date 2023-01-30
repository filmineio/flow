use serde::{Deserialize, Serialize};
use sled::{self, Error};
use crate::sync_store::config::SyncStoreConfig;

#[derive(Debug, Clone)]
pub struct SyncStore {
    db: sled::Db,
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncState {
    pub height: i64,
}


impl SyncStore {
    pub fn new(config: SyncStoreConfig) -> Result<SyncStore, Error> {
        // TODO: Use config builder
        match sled::open(config.path.as_str()) {
            Ok(db) => Ok(SyncStore { db }),
            Err(e) => Err(e)
        }
    }

    pub fn get_sync_state(&self) -> Option<SyncState> {
        match self.db.get(&"sync_state") {
            Ok(vec) => match vec {
                Some(v) => Some(bincode::deserialize(&v).unwrap()),
                _ => None
            }
            _ => None
        }
    }

    pub fn update_sync_state(&self, sync_state: SyncState) {
        let sync_state = bincode::serialize(&sync_state).unwrap();
        self.db.insert(&"sync_state", sync_state).unwrap();
    }

    pub fn get_current_height(&self) -> i64 {
        match self.get_sync_state() {
            Some(state) => state.height,
            _ => 0,
        }
    }

    pub fn update_current_height(&self, height: i64) {
        let mut sync_state = match self.get_sync_state() {
            Some(s) => s,
            _ => SyncState { height: 0 }
        };

        sync_state.height = height;
        self.update_sync_state(sync_state);
    }
}
