use crate::types::neardata_block_response_interface;
// ===========================================

pub struct ShardInfo {
    pub shard_id: u64,
    pub chunk_hash: Option<String>,
    pub height_created: Option<u64>,
    pub height_included: Option<u64>,
    pub gas_used: Option<u64>,
    pub gas_limit: Option<u64>,
    pub balance_burnt: Option<String>,
    pub validator_reward: Option<String>,
    pub rent_paid: Option<String>,
    pub tx_root: Option<String>,
    pub outcome_root: Option<String>,
    pub outgoing_receipts_root: Option<String>,
    pub prev_state_root: Option<String>,
    pub encoded_length: Option<u64>,
    pub signature: Option<String>,
    pub transaction_count: usize,
    pub receipt_execution_outcomes_count: usize,
    pub state_changes_count: usize,
}
// ===========================================

pub fn extract_shard_info_fun(block: &neardata_block_response_interface) -> Vec<ShardInfo> {
    block
        .shards
        .iter()
        .map(|shard| {
            let chunk_hash = shard.chunk.as_ref().map(|c| c.header.chunk_hash.clone());
            let height_created = shard.chunk.as_ref().map(|c| c.header.height_created);
            let height_included = shard.chunk.as_ref().map(|c| c.header.height_included);
            let gas_used = shard.chunk.as_ref().map(|c| c.header.gas_used);
            let gas_limit = shard.chunk.as_ref().map(|c| c.header.gas_limit);
            let balance_burnt = shard.chunk.as_ref().map(|c| c.header.balance_burnt.clone());
            let validator_reward = shard.chunk.as_ref().map(|c| c.header.validator_reward.clone());
            let rent_paid = shard.chunk.as_ref().map(|c| c.header.rent_paid.clone());
            let tx_root = shard.chunk.as_ref().map(|c| c.header.tx_root.clone());
            let outcome_root = shard.chunk.as_ref().map(|c| c.header.outcome_root.clone());
            let outgoing_receipts_root = shard
                .chunk
                .as_ref()
                .map(|c| c.header.outgoing_receipts_root.clone());
            let prev_state_root = shard.chunk.as_ref().map(|c| c.header.prev_state_root.clone());
            let encoded_length = shard.chunk.as_ref().map(|c| c.header.encoded_length);
            let signature = shard.chunk.as_ref().map(|c| c.header.signature.clone());
            let transaction_count = shard
                .chunk
                .as_ref()
                .map(|c| c.transactions.len())
                .unwrap_or(0);
            let receipt_execution_outcomes_count = shard.receipt_execution_outcomes.len();
            let state_changes_count = shard.state_changes.len();

            ShardInfo {
                shard_id: shard.shard_id,
                chunk_hash,
                height_created,
                height_included,
                gas_used,
                gas_limit,
                balance_burnt,
                validator_reward,
                rent_paid,
                tx_root,
                outcome_root,
                outgoing_receipts_root,
                prev_state_root,
                encoded_length,
                signature,
                transaction_count,
                receipt_execution_outcomes_count,
                state_changes_count,
            }
        })
        .collect()
}
// ===========================================
