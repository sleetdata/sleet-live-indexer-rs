// Types using near-primitives for NEAR blockchain data
#![allow(non_camel_case_types)]

use near_primitives::views::BlockHeaderView;
use serde::Deserialize;

// ===========================================
// NEARDATA API response structure
// This matches the response from https://mainnet.neardata.xyz/v0/last_block/final
#[derive(Deserialize, Debug)]
pub struct NeardataBlockResponse {
    pub block: NeardataBlock,
    pub shards: Vec<ShardData>,
}

// ===========================================
// Block structure - uses BlockHeaderView for header, custom for rest
#[derive(Deserialize, Debug)]
pub struct NeardataBlock {
    pub author: String,
    pub header: BlockHeaderView,
    pub chunks: Vec<NeardataChunk>,
}

// ===========================================
// Chunk structure matching neardata.xyz API
#[derive(Deserialize, Debug)]
pub struct NeardataChunk {
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
// Shard data containing chunk, transactions, receipts, and outcomes
#[derive(Deserialize, Debug)]
pub struct ShardData {
    pub shard_id: u64,
    pub chunk: ShardChunk,
    pub receipt_execution_outcomes: Vec<serde_json::Value>,
    pub state_changes: Vec<serde_json::Value>,
}

// ===========================================
// Chunk with transactions and receipts
#[derive(Deserialize, Debug)]
pub struct ShardChunk {
    pub author: String,
    pub header: NeardataChunkHeader,
    pub transactions: Vec<serde_json::Value>,
    pub receipts: Vec<serde_json::Value>,
    pub local_receipts: Vec<serde_json::Value>,
}

// ===========================================
// Shard chunk header
#[derive(Deserialize, Debug)]
pub struct NeardataChunkHeader {
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
    pub congestion_info: ChunkCongestionInfo,
    pub bandwidth_requests: serde_json::Value,
    pub signature: String,
}

// ===========================================
// Chunk congestion info
#[derive(Deserialize, Debug)]
pub struct ChunkCongestionInfo {
    pub delayed_receipts_gas: String,
    pub buffered_receipts_gas: String,
    pub receipt_bytes: u64,
    pub allowed_shard: u64,
}

// ===========================================
// Helper to get block height from response
impl NeardataBlockResponse {
    pub fn height(&self) -> u64 {
        self.block.header.height
    }

    pub fn hash(&self) -> String {
        self.block.header.hash.to_string()
    }

    pub fn author(&self) -> &str {
        &self.block.author
    }

    pub fn shard_count(&self) -> usize {
        self.shards.len()
    }
}

// ===========================================
// copyright 2026 by sleet.near
