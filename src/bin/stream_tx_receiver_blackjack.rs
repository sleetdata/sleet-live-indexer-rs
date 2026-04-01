// Stream all transactions where receiver_id is "blackjack-v2.warsofcards.near"
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    extract_block_info_fun::extract_block_info_fun,
    filter_by_receiver_fun::filter_by_receiver_fun,
    parse_block_fun::parse_block_fun,
};
use std::env;
use tokio_stream::StreamExt;
// ==============================================
const DEFAULT_NEAR_STREAM_URL: &str = "http://localhost:8080";
const TARGET_RECEIVER: &str = "blackjack-v2.warsofcards.near";
// ==============================================

#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("NEAR_STREAM_URL").unwrap_or_else(|_| DEFAULT_NEAR_STREAM_URL.to_string());

    println!("Connecting to NEAR Stream: {url}");
    println!("Filtering for receiver: {}\n", TARGET_RECEIVER);

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

                    // Filter by receiver using reusable function
                    let receiver_txs = filter_by_receiver_fun(&block, TARGET_RECEIVER);

                    if !receiver_txs.is_empty() {
                        println!("===========================================");
                        println!("Block #{} | Author: {}", block_info.height, block_info.author);
                        println!("===========================================");

                        for (i, tx) in receiver_txs.iter().enumerate() {
                            println!("\n[Tx #{}]", i + 1);
                            println!("  Shard: {}", tx.shard_id);
                            println!("  Tx Hash: {}", tx.tx_hash);
                            println!("  Signer: {}", tx.signer_id);
                            println!("  Receiver: {}", tx.receiver_id);
                            println!("  Actions: {}", tx.action_count);

                            if let Some(receipt_id) = &tx.receipt_id {
                                println!("  Receipt ID: {}", receipt_id);
                            }

                            if !tx.logs.is_empty() {
                                println!("  Logs:");
                                for log in &tx.logs {
                                    println!("    - {}", log);
                                }
                            }
                        }

                        println!("\n>>> Total txs to {} in this block: {}\n", TARGET_RECEIVER, receiver_txs.len());
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
