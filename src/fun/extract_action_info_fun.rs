use crate::types::neardata_action_interface;
// ===========================================

pub struct ActionInfo {
    pub action_type: String,
    pub method_name: Option<String>,
    pub args: Option<String>,
    pub deposit: Option<String>,
    pub gas: Option<u64>,
}
// ===========================================

pub fn extract_action_info_fun(action: &neardata_action_interface) -> ActionInfo {
    match action {
        neardata_action_interface::FunctionCall { FunctionCall } => ActionInfo {
            action_type: "FunctionCall".to_string(),
            method_name: Some(FunctionCall.method_name.clone()),
            args: Some(FunctionCall.args.clone()),
            deposit: Some(FunctionCall.deposit.clone()),
            gas: Some(FunctionCall.gas),
        },
        neardata_action_interface::Transfer { Transfer } => ActionInfo {
            action_type: "Transfer".to_string(),
            method_name: None,
            args: None,
            deposit: Some(Transfer.deposit.clone()),
            gas: None,
        },
        neardata_action_interface::CreateAccount { .. } => ActionInfo {
            action_type: "CreateAccount".to_string(),
            method_name: None,
            args: None,
            deposit: None,
            gas: None,
        },
        neardata_action_interface::DeleteAccount { DeleteAccount: _ } => ActionInfo {
            action_type: "DeleteAccount".to_string(),
            method_name: None,
            args: None,
            deposit: None,
            gas: None,
            // Could add beneficiary_id if needed
        },
        neardata_action_interface::AddKey { AddKey: _ } => ActionInfo {
            action_type: "AddKey".to_string(),
            method_name: None,
            args: None,
            deposit: None,
            gas: None,
        },
        neardata_action_interface::Delegate { Delegate: _ } => ActionInfo {
            action_type: "Delegate".to_string(),
            method_name: None,
            args: None,
            deposit: None,
            gas: None,
        },
        neardata_action_interface::String(_) => ActionInfo {
            action_type: "String".to_string(),
            method_name: None,
            args: None,
            deposit: None,
            gas: None,
        },
        neardata_action_interface::Any(_) => ActionInfo {
            action_type: "Any".to_string(),
            method_name: None,
            args: None,
            deposit: None,
            gas: None,
        },
    }
}
// ===========================================
