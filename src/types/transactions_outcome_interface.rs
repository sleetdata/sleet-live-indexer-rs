// Transaction outcome types for NEARDATA API
// Matches: neardata-types/src/interface/transactions_outcome_interface.ts
// ==============================================

use serde::{Deserialize, Serialize};

use super::receipts::neardata_receipt_interface;

// ===========================================
// ==== neardata_transactions_outcome_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_transactions_outcome_interface {
    pub execution_outcome: neardata_execution_outcome_interface,
    pub receipt: Option<neardata_receipt_interface>,
}

// ===========================================
// ==== neardata_execution_outcome_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_execution_outcome_interface {
    pub proof: Vec<neardata_merkle_proof_node_interface>,
    pub block_hash: String,
    pub id: String,
    pub outcome: neardata_outcome_interface,
}

// ===========================================
// ==== neardata_merkle_proof_node_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_merkle_proof_node_interface {
    pub hash: String,
    pub direction: String, // "Left" or "Right"
}

// ===========================================
// ==== neardata_outcome_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_outcome_interface {
    pub logs: Vec<String>,
    pub receipt_ids: Vec<String>,
    pub gas_burnt: u64,
    pub tokens_burnt: String,
    pub executor_id: String,
    pub status: neardata_outcome_status_interface,
    pub metadata: neardata_outcome_metadata_interface,
}

// ===========================================
// ==== neardata_outcome_status_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum neardata_outcome_status_interface {
    SuccessValue { SuccessValue: String },
    SuccessReceiptId { SuccessReceiptId: String },
    Failure { Failure: serde_json::Value },
}

// ===========================================
// ==== neardata_outcome_metadata_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_outcome_metadata_interface {
    pub version: u64,
    pub gas_profile: Option<Vec<neardata_gas_profile_entry_interface>>,
}

// ===========================================
// ==== neardata_gas_profile_entry_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_gas_profile_entry_interface {
    pub cost: String,
    pub cost_category: String,
    #[serde(default)]
    pub description: Option<String>,
}

// ===========================================
// copyright 2026 by sleet.near
