use crate::types::{
    neardata_action_interface, neardata_block_response_interface,
};
// ===========================================

pub struct TransferTransaction {
    pub shard_id: u64,
    pub tx_hash: String,
    pub signer_id: String,
    pub receiver_id: String,
    pub deposit: String,
    pub receipt_id: Option<String>,
    pub logs: Vec<String>,
}
// ===========================================

pub fn filter_transfer_actions_fun(
    block: &neardata_block_response_interface,
) -> Vec<TransferTransaction> {
    let mut transfers = Vec::new();

    for shard in &block.shards {
        let Some(chunk) = &shard.chunk else {
            continue;
        };

        for tx_with_outcome in &chunk.transactions {
            let tx = &tx_with_outcome.transaction;

            for action in &tx.actions {
                if let neardata_action_interface::Transfer { Transfer } = action {
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

                    transfers.push(TransferTransaction {
                        shard_id: shard.shard_id,
                        tx_hash: tx.hash.clone(),
                        signer_id: tx.signer_id.clone(),
                        receiver_id: tx.receiver_id.clone(),
                        deposit: Transfer.deposit.clone(),
                        receipt_id,
                        logs,
                    });
                }
            }
        }
    }

    transfers
}
// ===========================================
