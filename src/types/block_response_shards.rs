// Shard types for NEARDATA API
// Matches: neardata-types/src/interface/block_response_shards.ts
// ==============================================

use serde::{Deserialize, Serialize};

use super::receipts::{neardata_receipt_execution_outcome_interface, neardata_receipt_interface};
use super::transactions::neardata_transactions_interface;

// ===========================================
// ==== neardata_shard_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_shard_interface {
    pub shard_id: u64,
    pub chunk: Option<neardata_shard_chunk_interface>,
    pub receipt_execution_outcomes: Vec<neardata_receipt_execution_outcome_interface>,
    pub state_changes: Vec<neardata_state_change_interface>,
}

// ===========================================
// ==== neardata_state_change_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_state_change_interface {
    pub cause: neardata_state_change_cause_interface,
    pub change: serde_json::Value, // Flexible - varies by change type
    pub r#type: String,
}

// ===========================================
// ==== neardata_state_change_cause_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_state_change_cause_interface {
    pub receipt_hash: Option<String>,
    pub transaction_hash: Option<String>,
    pub r#type: String,
}

// ===========================================
// ==== neardata_shard_chunk_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_shard_chunk_interface {
    pub author: String,
    pub header: neardata_shard_chunk_header_interface,
    pub transactions: Vec<neardata_transactions_interface>,
    pub receipts: Vec<neardata_receipt_interface>,
    pub local_receipts: Vec<serde_json::Value>,
}

// ===========================================
// ==== neardata_shard_chunk_header_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_shard_chunk_header_interface {
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
    pub congestion_info: neardata_congestion_info_interface,
    pub bandwidth_requests: neardata_bandwidth_requests_interface,
    pub signature: String,
}

// ===========================================
// ==== neardata_congestion_info_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_congestion_info_interface {
    pub delayed_receipts_gas: String,
    pub buffered_receipts_gas: String,
    pub receipt_bytes: u64,
    pub allowed_shard: u64,
}

// ===========================================
// ==== neardata_bandwidth_requests_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_bandwidth_requests_interface {
    pub V1: neardata_bandwidth_requests_v1_interface,
}

// ===========================================
// ==== neardata_bandwidth_requests_v1_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_bandwidth_requests_v1_interface {
    pub requests: Vec<serde_json::Value>,
}

// ===========================================
// copyright 2026 by sleet.near
