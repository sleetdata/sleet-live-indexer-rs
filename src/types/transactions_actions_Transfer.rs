// Transfer action types
// Matches: neardata-types/src/interface/transactions_actions_Transfer.ts
// ==============================================

use serde::{Deserialize, Serialize};

// ===========================================
// ==== neardata_transfer_action_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_transfer_action_interface {
    pub deposit: String,
}

// ===========================================
// copyright 2026 by sleet.near
