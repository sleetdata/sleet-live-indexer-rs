// Stream and print only transactions with FunctionCall actions
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    extract_block_info_fun::extract_block_info_fun,
    filter_functioncall_actions_fun::filter_functioncall_actions_fun,
    parse_block_fun::parse_block_fun,
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

                    // Parse block using reusable function
                    let block = match parse_block_fun(&data) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Failed to parse block: {e}");
                            continue;
                        }
                    };

                    // Extract block info using reusable function
                    let block_info = extract_block_info_fun(&block);

                    // Filter FunctionCall actions using reusable function
                    let function_calls = filter_functioncall_actions_fun(&block);

                    if !function_calls.is_empty() {
                        println!("===========================================");
                        println!("Block #{} | Author: {}", block_info.height, block_info.author);
                        println!("===========================================");

                        for (i, function_call) in function_calls.iter().enumerate() {
                            println!("\n[FunctionCall #{}]", i + 1);
                            println!("  Shard: {}", function_call.shard_id);
                            println!("  Tx Hash: {}", function_call.tx_hash);
                            println!("  Signer: {}", function_call.signer_id);
                            println!("  Receiver: {}", function_call.receiver_id);
                            println!("  Method: {}", function_call.method_name);
                            println!("  Args: {}", function_call.args);
                            println!("  Deposit: {}", function_call.deposit);
                            println!("  Gas: {}", function_call.gas);

                            if let Some(receipt_id) = &function_call.receipt_id {
                                println!("  Receipt ID: {}", receipt_id);
                            }

                            if !function_call.logs.is_empty() {
                                println!("  Logs:");
                                for log in &function_call.logs {
                                    println!("    - {}", log);
                                }
                            }
                        }

                        println!("\n>>> Total FunctionCall txs in this block: {}\n", function_calls.len());
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
