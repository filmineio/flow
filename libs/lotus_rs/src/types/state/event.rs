use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Flags associated with an Event entry.
    #[derive(Deserialize, Serialize, Default)]
    #[serde(transparent)]
    pub struct Flags: u8 {
        const FLAG_INDEXED_KEY      = 0b00000001;
        const FLAG_INDEXED_VALUE    = 0b00000010;
        const FLAG_INDEXED_ALL      = Self::FLAG_INDEXED_KEY.bits | Self::FLAG_INDEXED_VALUE.bits;
    }
}

type ActorID = u64;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StampedEvent {
    /// Carries the ID of the actor that emitted this event.
    pub emitter: ActorID,
    /// The event as emitted by the actor.
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Entry {
    /// A bitmap conveying metadata or hints about this entry.
    pub flags: Flags,
    /// The key of this event.
    pub key: String,
    /// Any DAG-CBOR encodeable type.
    pub value: String,
}
