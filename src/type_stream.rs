// ===========================================
// Stream Block Event structure
#[derive(serde::Deserialize, Debug)]
pub struct STREAM_BLOCK_EVENT {
    pub block: STREAM_BLOCK,
    pub shards: Vec<STREAM_SHARD_DATA>,
}
// ===========================================
// Stream Block structure
#[derive(serde::Deserialize, Debug)]
pub struct STREAM_BLOCK {
    pub author: String,
    pub header: STREAM_BLOCK_HEADER,
}
// ===========================================
// Stream Block Header structure
#[derive(serde::Deserialize, Debug)]
pub struct STREAM_BLOCK_HEADER {
    pub height: u64,
    pub hash: String,
    pub prev_hash: String,
}
// ===========================================
// Stream Shard Data structure
#[derive(serde::Deserialize, Debug)]
pub struct STREAM_SHARD_DATA {
    pub chunk: Option<STREAM_CHUNK>,
    pub receipt_execution_outcomes: Vec<serde_json::Value>,
    pub shard_id: u64,
    pub state_changes: Vec<serde_json::Value>,
}
// ===========================================
// Stream Chunk structure
#[derive(serde::Deserialize, Debug)]
pub struct STREAM_CHUNK {
    pub header: STREAM_CHUNK_HEADER,
    pub transactions: Vec<serde_json::Value>,
}
// ===========================================
// Stream Chunk Header structure
#[derive(serde::Deserialize, Debug)]
pub struct STREAM_CHUNK_HEADER {
    pub chunk_hash: String,
    pub height_created: u64,
    pub height_included: u64,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub balance_burnt: String,
    pub validator_reward: String,
    pub rent_paid: String,
    pub tx_root: String,
    pub outcome_root: String,
    pub outgoing_receipts_root: String,
    pub prev_state_root: String,
    pub encoded_length: u64,
    pub signature: String,
}
// ===========================================
// copyright 2026 by sleet.near
