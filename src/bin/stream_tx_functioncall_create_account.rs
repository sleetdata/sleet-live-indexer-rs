// Stream and print only FunctionCall transactions with method_name "create_account"
// This is the most common way new NEAR accounts are created
// ==============================================
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::types::{
    neardata_action_interface, neardata_block_response_interface,
    neardata_outcome_status_interface,
};
use std::env;
use tokio_stream::StreamExt;
// ==============================================
const DEFAULT_NEAR_STREAM_URL: &str = "http://localhost:8080";
// ==============================================

#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("NEAR_STREAM_URL").unwrap_or_else(|_| DEFAULT_NEAR_STREAM_URL.to_string());

    println!("Connecting to NEAR Stream: {url}");
    println!("Filtering for create_account FunctionCalls...\n");

    let client = ClientBuilder::for_url(&url)?.build();
    let mut stream = client.stream();

    while let Some(event) = stream.next().await {
        match event {
            Ok(SSE::Event(ev)) => {
                let event_type = ev.event_type.as_str();

                if event_type == "block" {
                    let data = ev.data;

                    let block: neardata_block_response_interface =
                        match serde_json::from_str(&data) {
                            Ok(b) => b,
                            Err(e) => {
                                eprintln!("Failed to parse block: {e}");
                                continue;
                            }
                        };

                    let mut create_account_count = 0;

                    // Iterate through all shards and their transactions
                    for shard in &block.shards {
                        for tx_with_outcome in &shard.chunk.transactions {
                            let tx = &tx_with_outcome.transaction;

                            // Check for FunctionCall actions with method_name == "create_account"
                            for action in &tx.actions {
                                if let neardata_action_interface::FunctionCall { FunctionCall } = action {
                                    if FunctionCall.method_name == "create_account" {
                                        if create_account_count == 0 {
                                            println!("===========================================");
                                            println!("Block #{} | Author: {}", block.height(), block.author());
                                            println!("===========================================");
                                        }

                                        create_account_count += 1;

                                        // Decode args to get new_account_id and new_public_key
                                        let decoded_args = BASE64.decode(&FunctionCall.args)
                                            .ok()
                                            .and_then(|bytes| String::from_utf8(bytes).ok())
                                            .unwrap_or_else(|| FunctionCall.args.clone());

                                        println!("\n[create_account #{}]", create_account_count);
                                        println!("  Shard: {}", shard.shard_id);
                                        println!("  Tx Hash: {}", tx.hash);
                                        println!("  Signer: {}", tx.signer_id);
                                        println!("  Receiver: {}", tx.receiver_id);
                                        println!("  Gas: {}", FunctionCall.gas);
                                        println!("  Deposit: {}", FunctionCall.deposit);
                                        println!("  Decoded Args: {}", decoded_args);

                                        // Print outcome if available
                                        if let Some(receipt) = &tx_with_outcome.outcome.receipt {
                                            println!("  Receipt ID: {}", receipt.receipt_id);
                                        }

                                        // Print logs from outcome
                                        let outcome = &tx_with_outcome.outcome.execution_outcome.outcome;
                                        if !outcome.logs.is_empty() {
                                            println!("  Logs:");
                                            for log in &outcome.logs {
                                                println!("    - {}", log);
                                            }
                                        }

                                        // Print status
                                        match &outcome.status {
                                            neardata_outcome_status_interface::SuccessValue { SuccessValue } => {
                                                println!("  Status: Success ({})", SuccessValue);
                                            }
                                            neardata_outcome_status_interface::SuccessReceiptId { SuccessReceiptId } => {
                                                println!("  Status: SuccessReceiptId ({})", SuccessReceiptId);
                                            }
                                            neardata_outcome_status_interface::Failure { Failure } => {
                                                println!("  Status: Failure ({:?})", Failure);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if create_account_count > 0 {
                        println!("\n>>> Total create_account calls in this block: {}\n", create_account_count);
                    }
                }
            }
            Ok(_) => {}
            Err(err) => {
                eprintln!("Stream error: {err}");
                break;
            }
        }
    }

    Ok(())
}
// ==============================================
// copyright 2026 by sleet.near
