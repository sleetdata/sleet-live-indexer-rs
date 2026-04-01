use crate::fun::extract_action_info_fun::{extract_action_info_fun, ActionInfo};
use crate::types::neardata_block_response_interface;
// ===========================================

pub struct ReceiverTransaction {
    pub shard_id: u64,
    pub tx_hash: String,
    pub signer_id: String,
    pub receiver_id: String,
    pub actions: Vec<ActionInfo>,
    pub receipt_id: Option<String>,
    pub logs: Vec<String>,
}
// ===========================================

pub fn filter_by_receiver_fun(
    block: &neardata_block_response_interface,
    receiver_id: &str,
) -> Vec<ReceiverTransaction> {
    let mut transactions = Vec::new();

    for shard in &block.shards {
        let Some(chunk) = &shard.chunk else {
            continue;
        };

        for tx_with_outcome in &chunk.transactions {
            let tx = &tx_with_outcome.transaction;

            // Filter by receiver_id
            if tx.receiver_id == receiver_id {
                let logs = tx_with_outcome
                    .outcome
                    .execution_outcome
                    .outcome
                    .logs
                    .clone();

                let receipt_id = tx_with_outcome
                    .outcome
                    .receipt
                    .as_ref()
                    .map(|r| r.receipt_id.clone());

                // Extract action info for each action
                let actions: Vec<ActionInfo> = tx
                    .actions
                    .iter()
                    .map(|action| extract_action_info_fun(action))
                    .collect();

                transactions.push(ReceiverTransaction {
                    shard_id: shard.shard_id,
                    tx_hash: tx.hash.clone(),
                    signer_id: tx.signer_id.clone(),
                    receiver_id: tx.receiver_id.clone(),
                    actions,
                    receipt_id,
                    logs,
                });
            }
        }
    }

    transactions
}
// ===========================================
