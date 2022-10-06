use crate::types::chain::message::Message;
use crate::types::chain::signature::Signature;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignedMessage {
    Message: Message,
    Signature: Signature,
}
