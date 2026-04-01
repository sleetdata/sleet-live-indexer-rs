// Stream and print only transactions with DeleteAccount actions
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    extract_block_info_fun::extract_block_info_fun,
    filter_deleteaccount_actions_fun::filter_deleteaccount_actions_fun,
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
    println!("Filtering for DeleteAccount transactions...\n");

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

                    // Filter DeleteAccount actions using reusable function
                    let delete_accounts = filter_deleteaccount_actions_fun(&block);

                    if !delete_accounts.is_empty() {
                        println!("===========================================");
                        println!("Block #{} | Author: {}", block_info.height, block_info.author);
                        println!("===========================================");

                        for (i, delete_account) in delete_accounts.iter().enumerate() {
                            println!("\n[DeleteAccount #{}]", i + 1);
                            println!("  Shard: {}", delete_account.shard_id);
                            println!("  Tx Hash: {}", delete_account.tx_hash);
                            println!("  Signer: {}", delete_account.signer_id);
                            println!("  Receiver: {}", delete_account.receiver_id);
                            println!("  Public Key: {}", delete_account.public_key);
                            println!("  Nonce: {}", delete_account.nonce);
                            println!("  Priority Fee: {}", delete_account.priority_fee);
                            println!("  Signature: {}", delete_account.signature);
                            println!("  Beneficiary: {}", delete_account.beneficiary_id);

                            if let Some(receipt_id) = &delete_account.receipt_id {
                                println!("  Receipt ID: {}", receipt_id);
                            }

                            if !delete_account.logs.is_empty() {
                                println!("  Logs:");
                                for log in &delete_account.logs {
                                    println!("    - {}", log);
                                }
                            }
                        }

                        println!("\n>>> Total DeleteAccount txs in this block: {}\n", delete_accounts.len());
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
