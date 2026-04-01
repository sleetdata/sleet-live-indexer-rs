// Receipt types for NEARDATA API
// Matches: neardata-types/src/interface/receipts.ts
// ==============================================

use serde::{Deserialize, Serialize};

use super::transactions::{neardata_action_interface, neardata_data_receiver_interface};
use super::transactions_outcome_interface::neardata_execution_outcome_interface;

// ===========================================
// ==== neardata_receipt_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_receipt_interface {
    pub predecessor_id: String,
    pub receiver_id: String,
    pub receipt_id: String,
    pub receipt: neardata_receipt_kind_interface,
    pub priority: u64,
}

// ===========================================
// ==== neardata_receipt_kind_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum neardata_receipt_kind_interface {
    Action(neardata_action_receipt_interface),
    Data(neardata_data_receipt_interface),
}

// ===========================================
// ==== neardata_action_receipt_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_action_receipt_interface {
    pub signer_id: String,
    pub signer_public_key: String,
    pub gas_price: String,
    pub output_data_receivers: Vec<neardata_data_receiver_interface>,
    pub input_data_ids: Vec<String>,
    pub actions: Vec<neardata_action_interface>,
    pub is_promise_yield: bool,
}

// ===========================================
// ==== neardata_data_receipt_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_data_receipt_interface {
    pub data_id: String,
    pub data: Option<String>, // base64 encoded data, can be null
    pub is_promise_resume: bool,
}

// ===========================================
// ==== neardata_receipt_execution_outcome_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_receipt_execution_outcome_interface {
    pub execution_outcome: neardata_execution_outcome_interface,
    pub receipt: neardata_receipt_interface,
    pub tx_hash: String,
}

// ===========================================
// copyright 2026 by sleet.near
