use crate::types::{ActorBls, FlowBlock};
use crate::FlowMessage;
use anyhow::Result;
use lotus_rs::client::LotusClient;
use lotus_rs::types::chain::cid::{cid2str, CID};

use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

// TODO move to env
const NATIVE_ACTORS: [&str; 9] = [
    "t00", "t01", "t02", "t03", "t04", "t05", "t06", "t07", "t08",
];

pub async fn sync(
    client: &LotusClient,
    new_height: i64,
    map: &mut HashMap<String, Option<String>>,
) -> Result<i64> {
    let mut msg_map: HashMap<String, bool> = HashMap::new();
    let head = client.chain_get_tip_set_by_height(new_height).await?;
    for (inx, blk) in head.Cids.iter().enumerate() {
        let mut block_actors = HashSet::new();
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
            println!("{}", json!(&msg));

            if msg.Value.unwrap_or(0) > 0 {
                if msg.Addresses.RobustFrom.is_some() {
                    block_actors.insert(msg.Addresses.RobustFrom);
                }
                if msg.Addresses.RobustTo.is_some() {
                    block_actors.insert(msg.Addresses.RobustTo);
                }
            }
        }

        for actor_id in block_actors.into_iter().flatten() {
            if !NATIVE_ACTORS.contains(&actor_id.as_str()) {
                if let Ok(bls) = client
                    .state_get_actor(actor_id.clone(), Some(blk.clone()))
                    .await
                {
                    println!(
                        "{}",
                        json!(ActorBls {
                            Height: new_height,
                            Block: cid2str(blk.clone()),
                            ActorId: actor_id,
                            Balance: i64::from_str(&bls.Balance).unwrap_or(0),
                            Processed: blk_timestamp.clone()
                        })
                    )
                }
            }
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
        let mut f_msg = FlowMessage::from(t.ExecutionTrace.clone());
        f_msg.set_block(height, block_cid.clone());
        msgs.push(f_msg);
        t.ExecutionTrace.Subcalls.iter().for_each(|msg| {
            msg.iter().for_each(|v| {
                let cid: String = v.Msg.CID["/"].clone();
                if let std::collections::hash_map::Entry::Vacant(e) = msg_map.entry(cid) {
                    e.insert(true);
                    let mut f_msg = FlowMessage::from(v.clone());
                    f_msg.set_block(height, block_cid.clone());
                    f_msg.set_sub_calls_of(t.Msg.CID.clone());
                    msgs.push(f_msg);
                }
            });
        })
    });

    Ok((msgs, msg_map))
}

use std::time::{SystemTime, UNIX_EPOCH};
