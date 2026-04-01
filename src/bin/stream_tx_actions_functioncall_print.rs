// Stream and print only transactions with FunctionCall actions
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::types::{
    neardata_action_interface, neardata_block_response_interface,
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
    println!("Filtering for FunctionCall transactions...\n");

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

                    let mut functioncall_count = 0;

                    // Iterate through all shards and their transactions
                    for shard in &block.shards {
                        for tx_with_outcome in &shard.chunk.transactions {
                            let tx = &tx_with_outcome.transaction;

                            // Check if any action is FunctionCall
                            for action in &tx.actions {
                                if let neardata_action_interface::FunctionCall { FunctionCall } = action {
                                    if functioncall_count == 0 {
                                        println!("===========================================");
                                        println!("Block #{} | Author: {}", block.height(), block.author());
                                        println!("===========================================");
                                    }

                                    functioncall_count += 1;

                                    println!("\n[FunctionCall #{}]", functioncall_count);
                                    println!("  Shard: {}", shard.shard_id);
                                    println!("  Tx Hash: {}", tx.hash);
                                    println!("  Signer: {}", tx.signer_id);
                                    println!("  Receiver: {}", tx.receiver_id);
                                    println!("  Method: {}", FunctionCall.method_name);
                                    println!("  Args: {}", FunctionCall.args);
                                    println!("  Deposit: {}", FunctionCall.deposit);
                                    println!("  Gas: {}", FunctionCall.gas);

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
                                }
                            }
                        }
                    }

                    if functioncall_count > 0 {
                        println!("\n>>> Total FunctionCall txs in this block: {}\n", functioncall_count);
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
