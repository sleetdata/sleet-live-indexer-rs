// DeleteAccount action types
// Matches: neardata-types/src/interface/transactions_actions_DeleteAccount.ts
// ==============================================

use serde::{Deserialize, Serialize};

// ===========================================
// ==== neardata_delete_account_action_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_delete_account_action_interface {
    pub beneficiary_id: String,
}

// ===========================================
// copyright 2026 by sleet.near
