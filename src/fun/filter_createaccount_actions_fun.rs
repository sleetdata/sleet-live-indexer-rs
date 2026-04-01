use crate::types::{
    neardata_action_interface, neardata_block_response_interface,
};
// ===========================================

pub struct CreateAccountTransaction {
    pub shard_id: u64,
    pub tx_hash: String,
    pub signer_id: String,
    pub receiver_id: String,
    pub public_key: String,
    pub nonce: u64,
    pub priority_fee: u64,
    pub signature: String,
    pub receipt_id: Option<String>,
    pub logs: Vec<String>,
}
// ===========================================

pub fn filter_createaccount_actions_fun(
    block: &neardata_block_response_interface,
) -> Vec<CreateAccountTransaction> {
    let mut create_accounts = Vec::new();

    for shard in &block.shards {
        let Some(chunk) = &shard.chunk else {
            continue;
        };

        for tx_with_outcome in &chunk.transactions {
            let tx = &tx_with_outcome.transaction;

            for action in &tx.actions {
                if matches!(action, neardata_action_interface::CreateAccount { .. }) {
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

                    create_accounts.push(CreateAccountTransaction {
                        shard_id: shard.shard_id,
                        tx_hash: tx.hash.clone(),
                        signer_id: tx.signer_id.clone(),
                        receiver_id: tx.receiver_id.clone(),
                        public_key: tx.public_key.clone(),
                        nonce: tx.nonce,
                        priority_fee: tx.priority_fee.clone(),
                        signature: tx.signature.clone(),
                        receipt_id,
                        logs,
                    });
                }
            }
        }
    }

    create_accounts
}
// ===========================================
