// Transaction types for NEARDATA API
// Matches: neardata-types/src/interface/transactions.ts
// ==============================================

use serde::{Deserialize, Serialize};

use super::transactions_outcome_interface::neardata_transactions_outcome_interface;
use super::transactions_actions_AddKey::neardata_add_key_action_interface;
use super::transactions_actions_CreateAccount::neardata_create_account_action_interface;
use super::transactions_actions_Delegate::neardata_delegate_action_interface;
use super::transactions_actions_DeleteAccount::neardata_delete_account_action_interface;
use super::transactions_actions_FunctionCall::neardata_function_call_action_interface;
use super::transactions_actions_Transfer::neardata_transfer_action_interface;

// ===========================================
// ==== neardata_transactions_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_transactions_interface {
    pub transaction: neardata_transactions_transaction_interface,
    pub outcome: neardata_transactions_outcome_interface,
}

// ===========================================
// ==== neardata_transactions_transaction_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_transactions_transaction_interface {
    pub signer_id: String,
    pub public_key: String,
    pub nonce: u64,
    pub receiver_id: String,
    pub actions: Vec<neardata_action_interface>,
    pub priority_fee: u64,
    pub signature: String,
    pub hash: String,
}

// ===========================================
// ==== neardata_action_interface ====
/// Actions are objects with a single PascalCase key
/// Example: {"Delegate": {...}} or {"FunctionCall": {...}}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum neardata_action_interface {
    /// String action (rare, for simple actions)
    String(String),
    /// CreateAccount action
    CreateAccount { CreateAccount: neardata_create_account_action_interface },
    /// DeleteAccount action
    DeleteAccount { DeleteAccount: neardata_delete_account_action_interface },
    /// AddKey action
    AddKey { AddKey: neardata_add_key_action_interface },
    /// FunctionCall action
    FunctionCall { FunctionCall: neardata_function_call_action_interface },
    /// Transfer action
    Transfer { Transfer: neardata_transfer_action_interface },
    /// Delegate action
    Delegate { Delegate: neardata_delegate_action_interface },
}

// ===========================================
// ==== neardata_data_receiver_interface ====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct neardata_data_receiver_interface {
    pub data_id: String,
    pub receiver_id: String,
}

// ===========================================
// copyright 2026 by sleet.near
