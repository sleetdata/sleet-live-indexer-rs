// Block chunks types for NEARDATA API
// Matches: neardata-types/src/interface/block_response_block_chunks.ts
// ==============================================

use serde::{Deserialize, Serialize};

// ===========================================
// ==== neardata_block_chunks_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_block_chunks_interface {
    pub chunk_hash: String,
    pub prev_block_hash: String,
    pub outcome_root: String,
    pub prev_state_root: String,
    pub encoded_merkle_root: String,
    pub encoded_length: u64,
    pub height_created: u64,
    pub height_included: u64,
    pub shard_id: u64,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub rent_paid: String,
    pub validator_reward: String,
    pub balance_burnt: String,
    pub outgoing_receipts_root: String,
    pub tx_root: String,
    pub validator_proposals: Vec<serde_json::Value>,
    pub congestion_info: serde_json::Value,
    pub bandwidth_requests: serde_json::Value,
    pub signature: String,
}

// ===========================================
// copyright 2026 by sleet.near
