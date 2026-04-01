// Delegate action types
// Matches: neardata-types/src/interface/transactions_actions_Delegate.ts
// ==============================================

use serde::{Deserialize, Serialize};

use crate::types::transactions::neardata_action_interface;

// ===========================================
// ==== neardata_delegate_action_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_delegate_action_interface {
    pub delegate_action: neardata_delegate_action_content_interface,
    pub signature: String,
}

// ===========================================
// ==== neardata_delegate_action_content_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_delegate_action_content_interface {
    pub actions: Vec<neardata_action_interface>,
    pub max_block_height: u64,
    pub nonce: u64,
    pub public_key: String,
    pub receiver_id: String,
    pub sender_id: String,
}

// ===========================================
// copyright 2026 by sleet.near
