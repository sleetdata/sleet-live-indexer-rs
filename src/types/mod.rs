// Type definitions for NEARDATA API
// Matches structure of neardata-types/src/interface/index.ts
// ==============================================
//
// Usage:
// ```rust
// use sleet_live_indexer_rs::types::block_response::neardata_block_response_interface;
// ```

#![allow(non_snake_case)]

// Block response types (main entry point)
pub mod block_response;
pub mod block_response_block_chunks;
pub mod block_response_shards;

// Receipt types
pub mod receipts;

// Transaction types
pub mod transactions;
pub mod transactions_outcome_interface;

// Transaction action types
pub mod transactions_actions_AddKey;
pub mod transactions_actions_CreateAccount;
pub mod transactions_actions_Delegate;
pub mod transactions_actions_DeleteAccount;
pub mod transactions_actions_FunctionCall;
pub mod transactions_actions_Transfer;

// ===========================================
// Re-export commonly used types at module root
// ===========================================

// Block response
pub use block_response::{
    neardata_block_header_interface,
    neardata_block_interface,
    neardata_block_response_interface,
};

// Shards
pub use block_response_shards::{
    neardata_shard_chunk_header_interface,
    neardata_shard_chunk_interface,
    neardata_shard_interface,
    neardata_state_change_interface,
};

// Chunks
pub use block_response_block_chunks::neardata_block_chunks_interface;

// Receipts
pub use receipts::{
    neardata_action_receipt_interface,
    neardata_data_receipt_interface,
    neardata_receipt_execution_outcome_interface,
    neardata_receipt_interface,
    neardata_receipt_kind_interface,
};

// Transactions
pub use transactions::{
    neardata_action_interface,
    neardata_data_receiver_interface,
    neardata_transactions_interface,
    neardata_transactions_transaction_interface,
};

// Outcomes
pub use transactions_outcome_interface::{
    neardata_execution_outcome_interface,
    neardata_outcome_interface,
    neardata_outcome_status_interface,
    neardata_transactions_outcome_interface,
};

// Actions
pub use transactions_actions_AddKey::neardata_add_key_action_interface;
pub use transactions_actions_CreateAccount::neardata_create_account_action_interface;
pub use transactions_actions_Delegate::neardata_delegate_action_interface;
pub use transactions_actions_DeleteAccount::neardata_delete_account_action_interface;
pub use transactions_actions_FunctionCall::neardata_function_call_action_interface;
pub use transactions_actions_Transfer::neardata_transfer_action_interface;

// ===========================================
// copyright 2026 by sleet.near
