// Block response types for NEARDATA API
// Matches: neardata-types/src/interface/block_response.ts
// ==============================================

use serde::{Deserialize, Serialize};

use super::block_response_block_chunks::neardata_block_chunks_interface;
use super::block_response_shards::neardata_shard_interface;

// ===========================================
// ==== neardata_block_response_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_block_response_interface {
    pub block: neardata_block_interface,
    pub shards: Vec<neardata_shard_interface>,
}

// ===========================================
// ==== neardata_block_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_block_interface {
    pub author: String,
    pub header: neardata_block_header_interface,
    pub chunks: Vec<neardata_block_chunks_interface>,
}

// ===========================================
// ==== neardata_block_header_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_block_header_interface {
    pub height: u64,
    pub prev_height: u64,
    pub epoch_id: String,
    pub next_epoch_id: String,
    pub hash: String,
    pub prev_hash: String,
    pub prev_state_root: String,
    pub block_body_hash: String,
    pub chunk_receipts_root: String,
    pub chunk_headers_root: String,
    pub chunk_tx_root: String,
    pub outcome_root: String,
    pub chunks_included: u64,
    pub challenges_root: String,
    pub timestamp: u64,
    pub timestamp_nanosec: String,
    pub random_value: String,
    pub validator_proposals: Vec<serde_json::Value>,
    pub chunk_mask: Vec<bool>,
    pub gas_price: String,
    pub block_ordinal: u64,
    pub rent_paid: String,
    pub validator_reward: String,
    pub total_supply: String,
    pub challenges_result: Vec<serde_json::Value>,
    pub last_final_block: String,
    pub last_ds_final_block: String,
    pub next_bp_hash: String,
    pub block_merkle_root: String,
    pub epoch_sync_data_hash: Option<String>,
    pub approvals: Vec<Option<String>>,
    pub signature: String,
    pub latest_protocol_version: u64,
    pub chunk_endorsements: Vec<Vec<u8>>,
}

// ===========================================
// Helper methods for neardata_block_response_interface
impl neardata_block_response_interface {
    pub fn height(&self) -> u64 {
        self.block.header.height
    }

    pub fn hash(&self) -> &str {
        &self.block.header.hash
    }

    pub fn author(&self) -> &str {
        &self.block.author
    }

    pub fn shard_count(&self) -> usize {
        self.shards.len()
    }

    pub fn chunk_count(&self) -> usize {
        self.block.chunks.len()
    }
}

// ===========================================
// copyright 2026 by sleet.near
