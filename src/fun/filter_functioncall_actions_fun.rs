use crate::types::{
    neardata_action_interface, neardata_block_response_interface,
};
// ===========================================

pub struct FunctionCallTransaction {
    pub shard_id: u64,
    pub tx_hash: String,
    pub signer_id: String,
    pub receiver_id: String,
    pub method_name: String,
    pub args: String,
    pub deposit: String,
    pub gas: u64,
    pub receipt_id: Option<String>,
    pub logs: Vec<String>,
}
// ===========================================

pub fn filter_functioncall_actions_fun(
    block: &neardata_block_response_interface,
) -> Vec<FunctionCallTransaction> {
    let mut function_calls = Vec::new();

    for shard in &block.shards {
        let Some(chunk) = &shard.chunk else {
            continue;
        };

        for tx_with_outcome in &chunk.transactions {
            let tx = &tx_with_outcome.transaction;

            for action in &tx.actions {
                if let neardata_action_interface::FunctionCall { FunctionCall } = action {
                    let logs = tx_with_outcome
                        .outcome
                        .execution_outcome
                        .outcome
                        .logs
                        .clone();

                    let receipt_id = tx_with_outcome
                        .outcome
                        .receipt
                        .as_ref()
                        .map(|r| r.receipt_id.clone());

                    function_calls.push(FunctionCallTransaction {
                        shard_id: shard.shard_id,
                        tx_hash: tx.hash.clone(),
                        signer_id: tx.signer_id.clone(),
                        receiver_id: tx.receiver_id.clone(),
                        method_name: FunctionCall.method_name.clone(),
                        args: FunctionCall.args.clone(),
                        deposit: FunctionCall.deposit.clone(),
                        gas: FunctionCall.gas,
                        receipt_id,
                        logs,
                    });
                }
            }
        }
    }

    function_calls
}
// ===========================================
