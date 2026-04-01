// Stream and print only FunctionCall transactions with method_name "create_account"
// This is the most common way new NEAR accounts are created
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    decode_base64_args_fun::decode_base64_args_fun,
    extract_block_info_fun::extract_block_info_fun,
    filter_method_functioncall_fun::filter_method_functioncall_fun,
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
    println!("Filtering for create_account FunctionCalls...\n");

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

                    // Filter create_account method calls using reusable function
                    let create_account_calls = filter_method_functioncall_fun(&block, "create_account");

                    if !create_account_calls.is_empty() {
                        println!("===========================================");
                        println!("Block #{} | Author: {}", block_info.height, block_info.author);
                        println!("===========================================");

                        for (i, create_account_call) in create_account_calls.iter().enumerate() {
                            // Decode args using reusable function
                            let decoded_args = decode_base64_args_fun(&create_account_call.args)
                                .unwrap_or_else(|| create_account_call.args.clone());

                            println!("\n[create_account #{}]", i + 1);
                            println!("  Shard: {}", create_account_call.shard_id);
                            println!("  Tx Hash: {}", create_account_call.tx_hash);
                            println!("  Signer: {}", create_account_call.signer_id);
                            println!("  Receiver: {}", create_account_call.receiver_id);
                            println!("  Gas: {}", create_account_call.gas);
                            println!("  Deposit: {}", create_account_call.deposit);
                            println!("  Decoded Args: {}", decoded_args);

                            if let Some(receipt_id) = &create_account_call.receipt_id {
                                println!("  Receipt ID: {}", receipt_id);
                            }

                            // Extract outcome info using reusable function
                            // Note: We need to get this from the original block data
                            if !create_account_call.logs.is_empty() {
                                println!("  Logs:");
                                for log in &create_account_call.logs {
                                    println!("    - {}", log);
                                }
                            }
                        }

                        println!("\n>>> Total create_account calls in this block: {}\n", create_account_calls.len());
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
