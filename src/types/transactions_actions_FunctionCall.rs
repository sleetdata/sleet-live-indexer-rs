// FunctionCall action types
// Matches: neardata-types/src/interface/transactions_actions_FunctionCall.ts
// ==============================================

use serde::{Deserialize, Serialize};

// ===========================================
// ==== neardata_function_call_action_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_function_call_action_interface {
    pub deposit: String,
    pub gas: u64,
    pub method_name: String,
    pub args: String, // base64 encoded
}

// ===========================================
// copyright 2026 by sleet.near
