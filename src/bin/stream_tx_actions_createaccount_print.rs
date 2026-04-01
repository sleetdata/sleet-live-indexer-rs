// Stream and print only transactions with CreateAccount actions
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    extract_block_info_fun::extract_block_info_fun,
    filter_createaccount_actions_fun::filter_createaccount_actions_fun,
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
    println!("Filtering for CreateAccount transactions...\n");

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

                    // Filter CreateAccount actions using reusable function
                    let create_accounts = filter_createaccount_actions_fun(&block);

                    if !create_accounts.is_empty() {
                        println!("===========================================");
                        println!("Block #{} | Author: {}", block_info.height, block_info.author);
                        println!("===========================================");

                        for (i, create_account) in create_accounts.iter().enumerate() {
                            println!("\n[CreateAccount #{}]", i + 1);
                            println!("  Shard: {}", create_account.shard_id);
                            println!("  Tx Hash: {}", create_account.tx_hash);
                            println!("  Signer: {}", create_account.signer_id);
                            println!("  Receiver: {}", create_account.receiver_id);
                            println!("  Public Key: {}", create_account.public_key);
                            println!("  Nonce: {}", create_account.nonce);
                            println!("  Priority Fee: {}", create_account.priority_fee);
                            println!("  Signature: {}", create_account.signature);

                            if let Some(receipt_id) = &create_account.receipt_id {
                                println!("  Receipt ID: {}", receipt_id);
                            }

                            if !create_account.logs.is_empty() {
                                println!("  Logs:");
                                for log in &create_account.logs {
                                    println!("    - {}", log);
                                }
                            }
                        }

                        println!("\n>>> Total CreateAccount txs in this block: {}\n", create_accounts.len());
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
