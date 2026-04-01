// Stream and print only transactions with Transfer actions
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    extract_block_info_fun::extract_block_info_fun,
    filter_transfer_actions_fun::filter_transfer_actions_fun,
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
    println!("Filtering for Transfer transactions...\n");

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

                    // Filter transfer actions using reusable function
                    let transfers = filter_transfer_actions_fun(&block);

                    if !transfers.is_empty() {
                        println!("===========================================");
                        println!("Block #{} | Author: {}", block_info.height, block_info.author);
                        println!("===========================================");

                        for (i, transfer) in transfers.iter().enumerate() {
                            println!("\n[Transfer #{}]", i + 1);
                            println!("  Shard: {}", transfer.shard_id);
                            println!("  Tx Hash: {}", transfer.tx_hash);
                            println!("  Signer: {}", transfer.signer_id);
                            println!("  Receiver: {}", transfer.receiver_id);
                            println!("  Deposit: {} yoctoNEAR", transfer.deposit);

                            if let Some(receipt_id) = &transfer.receipt_id {
                                println!("  Receipt ID: {}", receipt_id);
                            }

                            if !transfer.logs.is_empty() {
                                println!("  Logs:");
                                for log in &transfer.logs {
                                    println!("    - {}", log);
                                }
                            }
                        }

                        println!("\n>>> Total Transfer txs in this block: {}\n", transfers.len());
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
