// AddKey action types
// Matches: neardata-types/src/interface/transactions_actions_AddKey.ts
// ==============================================

use serde::{Deserialize, Serialize};

// ===========================================
// ==== neardata_add_key_action_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_add_key_action_interface {
    pub access_key: neardata_access_key_interface,
    pub public_key: String,
}

// ===========================================
// ==== neardata_access_key_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_access_key_interface {
    pub nonce: u64,
    pub permission: neardata_access_key_permission_interface,
}

// ===========================================
// ==== neardata_access_key_permission_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum neardata_access_key_permission_interface {
    FullAccess(String), // "FullAccess"
    FunctionCall(neardata_function_call_permission_interface),
}

// ===========================================
// ==== neardata_function_call_permission_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_function_call_permission_interface {
    pub allowance: Option<String>,
    pub receiver_id: String,
    pub method_names: Vec<String>,
}

// ===========================================
// copyright 2026 by sleet.near
