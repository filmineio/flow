use crate::types::{FlowBlock, FlowEvent};
use crate::FlowMessage;
use anyhow::Result;
use lotus_rs::client::LotusClient;
use lotus_rs::types::chain::cid::{str2cid, CID};

use serde_json::json;
use std::collections::HashMap;

pub async fn sync(
    client: &LotusClient,
    new_height: i64,
    map: &mut HashMap<String, Option<String>>,
) -> Result<i64> {
    let mut msg_map: HashMap<String, bool> = HashMap::new();
    let head = client.chain_get_tip_set_by_height(new_height).await?;
    for (inx, blk) in head.Cids.iter().enumerate() {
        let mut blk_timestamp = "".to_string();
        if let Some(block) = head.Blocks.get(inx) {
            let mut f_blk = FlowBlock::from(block.clone());
            f_blk.set_cid(blk.clone());
            blk_timestamp = f_blk.Block.Timestamp.to_string();
            println!("{}", json!(f_blk));
        }
        let (msgs, b) = get_block_state(client, new_height, msg_map.clone(), blk.clone()).await?;
        msg_map = b;

        for mut msg in msgs {
            msg.resolve_addresses(client, map).await;
            msg.set_block_timestamp(blk_timestamp.clone());

            if let Some(rct) = &msg.MessageRct {
                if let Some(ev_root) = &rct.EventsRoot {
                    let v = client.chain_get_events(str2cid(ev_root.clone())).await;
                    if let Ok(v) = v {
                        let mut idx = 0;
                        for e in &v {
                            let ev = FlowEvent::from((
                                msg.Cid.clone(),
                                ev_root.clone(),
                                e.emitter,
                                idx,
                                e.entries.clone(),
                            ));
                            idx += 1;
                            println!("{}", json!(ev));
                        }
                        msg.set_number_of_events(v.len() as i64);
                    }
                }
            }
            println!("{}", json!(&msg));
        }
    }

    Ok(new_height + 1)
}

pub async fn get_block_state(
    client: &LotusClient,
    height: i64,
    mut msg_map: HashMap<String, bool>,
    block_cid: CID,
) -> Result<(Vec<FlowMessage>, HashMap<String, bool>)> {
    let state = client
        .state_compute(height, vec![], vec![block_cid.clone()])
        .await?;
    let mut msgs: Vec<FlowMessage> = vec![];

    state.Trace.iter().for_each(|t| {
        let mut f_msg = FlowMessage::from((t.ExecutionTrace.clone(), t.MsgCid.clone()["/"].clone()));
        f_msg.set_block(height, block_cid.clone());
        msgs.push(f_msg);
        t.ExecutionTrace.Subcalls.iter().for_each(|msg| {
            msg.iter().for_each(|v| {
                let cid: String = v.Msg.CID["/"].clone();
                if let std::collections::hash_map::Entry::Vacant(e) = msg_map.entry(cid) {
                    e.insert(true);
                    let mut f_msg = FlowMessage::from((v.clone(), cid.clone()));
                    f_msg.set_block(height, block_cid.clone());
                    f_msg.set_sub_calls_of(t.Msg.CID.clone());
                    msgs.push(f_msg);
                }
            });
        })
    });

    Ok((msgs, msg_map))
}
