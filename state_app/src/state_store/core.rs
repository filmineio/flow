use sled;

use crate::state_store::{
    config::StateStoreConfig,
    types::{StateStoreError, SyncState},
};

#[derive(Debug)]
pub struct StateStore {
    db: sled::Db,
}

impl StateStore {
    pub fn new(config: StateStoreConfig) -> Result<StateStore, StateStoreError> {
        let db = sled::open(config.path.as_str())?;

        Ok(Self { db })
    }

    pub async fn get_sync_state(&self) -> Option<SyncState> {
        let sync_state = match self.db.get(&"sync_state") {
            Ok(vec) => match vec {
                Some(vec) => vec,
                _ => return None,
            },
            _ => return None,
        };

        // If we fail to deserialize there is possibly some corrupted data
        // in that case we return None so we can override it
        match bincode::deserialize(&sync_state) {
            Ok(sync_state) => Some(sync_state),
            _ => None,
        }
    }

    pub async fn update_sync_state(&self, sync_state: SyncState) -> Result<(), StateStoreError> {
        let sync_state = bincode::serialize(&sync_state)?;

        self.db.insert(&"sync_state", sync_state)?;
        self.db.flush_async().await?;

        Ok(())
    }

    pub async fn get_current_height(&self) -> i64 {
        match self.get_sync_state().await {
            Some(state) => state.height,
            _ => 0,
        }
    }

    pub async fn update_current_height(&self, height: i64) -> Result<(), StateStoreError> {
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

    // updates initial height if current height is zero and initial height is higher then 0
    pub async fn update_initial_height(&self, initial_height: i64) -> Result<(), StateStoreError> {
        let current_height = self.get_current_height().await;

        if current_height == 0 && initial_height > 0 {
            return self.update_current_height(initial_height).await;
        }

        Ok(())
    }
}
