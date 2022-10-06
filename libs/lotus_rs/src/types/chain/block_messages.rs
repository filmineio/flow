use crate::types::chain::message::Message;
use crate::types::chain::signed_message::SignedMessage;
use crate::CID;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockMessages {
    pub BlsMessages: Vec<Message>,
    pub Cids: Vec<CID>,
    pub SecpkMessages: Vec<SignedMessage>,
}
