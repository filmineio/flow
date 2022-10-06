use crate::types::FlowBlock;
use crate::FlowMessage;
use anyhow::Result;
use lotus_rs::client::LotusClient;
use lotus_rs::types::chain::cid::CID;
use serde_json::json;
use serde_json::Value::Null;
use std::collections::HashMap;

pub async fn sync(client: &LotusClient, new_height: i64) -> Result<i64> {
    let mut msg_map: HashMap<String, bool> = HashMap::new();
    let head = client.chain_get_tip_set_by_height(new_height).await?;
    for (inx, blk) in head.Cids.iter().enumerate() {
        if let Some(block) = head.Blocks.get(inx) {
            let mut f_blk = FlowBlock::from(block.clone());
            f_blk.set_cid(blk.clone());
            println!("{}", json!(f_blk));
        }
        let (msgs, b) = get_block_state(client, new_height, msg_map.clone(), blk.clone()).await?;
        msg_map = b;

        for mut msg in msgs {
            if let Some(params) = msg.Message.Params.clone() {
                let decoded = client
                    .state_decode_params(msg.Message.To.clone(), msg.Message.Method, params)
                    .await;

                match decoded {
                    Ok(d) => msg.set_decoded_params(d),
                    Err(_e) => msg.set_decoded_params(Null),
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
