use crate::resources::event::types::{EthEvent, Event};
use lotus_rs::types::state::event::{Entry, StampedEvent};
use serde_json::json;
use web3::ethabi::{Hash, RawLog};

pub fn decode_event(ev: &Event) -> EthEvent {
    decode_inner(ev)
}

fn decode_inner(ev: &Event) -> EthEvent {
    let mut log = EthEvent::default();
    for entry in &ev.entries {
        let value: Vec<u8> = base64::decode(&entry.value)
            .unwrap()
            .iter()
            .map(|v| *v)
            .skip(2)
            .collect::<Vec<u8>>();
        let value: &[u8] = &value;
        let mut b = web3::types::Bytes::default();
        if entry.key.contains("topic") {
            b.0.extend_from_slice(value);
            log.topics.push(b)
        } else {
            b.0.extend_from_slice(value);
            log.data = b
        }
    }
    log.order = ev.order;

    log
}
