use crate::fun::filter_functioncall_actions_fun::{
    filter_functioncall_actions_fun, FunctionCallTransaction,
};
use crate::types::neardata_block_response_interface;
// ===========================================

pub fn filter_method_functioncall_fun(
    block: &neardata_block_response_interface,
    method_name: &str,
) -> Vec<FunctionCallTransaction> {
    let all_function_calls = filter_functioncall_actions_fun(block);

    all_function_calls
        .into_iter()
        .filter(|fc| fc.method_name == method_name)
        .collect()
}
// ===========================================
