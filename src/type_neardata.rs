// ===========================================
// NEARDATA API response structure
#[derive(serde::Deserialize, Debug)]
pub struct NEARDATA_BLOCK_RESPONSE {
    pub block: NEARDATA_BLOCK,
}
// ===========================================
// NEARDATA Block structure
#[derive(serde::Deserialize, Debug)]
pub struct NEARDATA_BLOCK {
    pub author: String,
    pub chunks: Vec<NEARDATA_CHUNK>,
    pub header: NEARDATA_BLOCK_HEADER,
}
// ===========================================
// NEARDATA Block Header structure
#[derive(serde::Deserialize, Debug)]
pub struct NEARDATA_BLOCK_HEADER {
    pub height: u64,
    pub hash: String,
    pub prev_hash: String,
}
// ===========================================
// NEARDATA Chunk structure
#[derive(serde::Deserialize, Debug)]
pub struct NEARDATA_CHUNK {
    pub shard_id: u64,
    pub chunk_hash: String,
    pub height_created: u64,
    pub height_included: u64,
    pub encoded_length: u64,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub signature: String,
    pub balance_burnt: String,
    pub validator_reward: String,
    pub rent_paid: String,
}
// ===========================================
// copyright 2026 by sleet.near
