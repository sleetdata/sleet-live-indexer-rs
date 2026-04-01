use crate::types::{
    neardata_outcome_status_interface, neardata_transactions_outcome_interface,
};
// ===========================================

pub struct OutcomeInfo {
    pub logs: Vec<String>,
    pub status: String,
}
// ===========================================

pub fn extract_outcome_info_fun(
    tx_with_outcome: &neardata_transactions_outcome_interface,
) -> OutcomeInfo {
    let outcome = &tx_with_outcome.execution_outcome.outcome;

    let status = match &outcome.status {
        neardata_outcome_status_interface::SuccessValue { SuccessValue } => {
            format!("Success ({})", SuccessValue)
        }
        neardata_outcome_status_interface::SuccessReceiptId { SuccessReceiptId } => {
            format!("SuccessReceiptId ({})", SuccessReceiptId)
        }
        neardata_outcome_status_interface::Failure { Failure } => {
            format!("Failure ({:?})", Failure)
        }
    };

    OutcomeInfo {
        logs: outcome.logs.clone(),
        status,
    }
}
// ===========================================
