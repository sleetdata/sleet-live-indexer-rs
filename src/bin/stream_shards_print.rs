use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    extract_block_info_fun::extract_block_info_fun,
    extract_shard_info_fun::extract_shard_info_fun,
    parse_block_fun::parse_block_fun,
};
use std::env;
use tokio_stream::StreamExt;
// ===========================================
const DEFAULT_NEAR_STREAM_URL: &str = "http://localhost:8080";
// ===========================================
// main
#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("NEAR_STREAM_URL").unwrap_or_else(|_| DEFAULT_NEAR_STREAM_URL.to_string());

    println!("Connecting to NEAR Stream: {url}");

    let client = ClientBuilder::for_url(&url)?.build();
    let mut stream = client.stream();

    while let Some(event) = stream.next().await {
        match event {
            Ok(SSE::Event(ev)) => {
                let event_type = ev.event_type.as_str();

                if event_type == "block" {
                    let data = ev.data;

                    // Parse block using reusable function
                    let block_event = match parse_block_fun(&data) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Failed to parse block: {e}");
                            continue;
                        }
                    };

                    // Extract block info using reusable function
                    let block_info = extract_block_info_fun(&block_event);

                    println!("===============================");
                    println!("Block #{}", block_info.height);
                    println!("Author: {}", block_info.author);
                    println!("Hash: {}", block_info.hash);
                    println!("Shards: {}", block_info.shard_count);
                    println!("===============================");

                    // Extract shard info using reusable function
                    let shard_infos = extract_shard_info_fun(&block_event);

                    for shard_info in &shard_infos {
                        println!("--- Shard #{} ---", shard_info.shard_id);

                        if let Some(chunk_hash) = &shard_info.chunk_hash {
                            println!("  Chunk Hash: {}", chunk_hash);
                            println!("  Height Created: {}", shard_info.height_created.unwrap_or(0));
                            println!("  Height Included: {}", shard_info.height_included.unwrap_or(0));
                            println!("  Gas Used: {}", shard_info.gas_used.unwrap_or(0));
                            println!("  Gas Limit: {}", shard_info.gas_limit.unwrap_or(0));
                            println!("  Balance Burnt: {}", &shard_info.balance_burnt.clone().unwrap_or_default());
                            println!("  Validator Reward: {}", &shard_info.validator_reward.clone().unwrap_or_default());
                            println!("  Rent Paid: {}", &shard_info.rent_paid.clone().unwrap_or_default());
                            println!("  Tx Root: {}", &shard_info.tx_root.clone().unwrap_or_default());
                            println!("  Outcome Root: {}", &shard_info.outcome_root.clone().unwrap_or_default());
                            println!(
                                "  Outgoing Receipts Root: {}",
                                &shard_info.outgoing_receipts_root.clone().unwrap_or_default()
                            );
                            println!("  Prev State Root: {}", &shard_info.prev_state_root.clone().unwrap_or_default());
                            println!("  Encoded Length: {}", shard_info.encoded_length.unwrap_or(0));
                            println!("  Signature: {}", &shard_info.signature.clone().unwrap_or_default());
                            println!("  Transactions: {}", shard_info.transaction_count);

                            println!(
                                "  Receipt Execution Outcomes: {}",
                                shard_info.receipt_execution_outcomes_count
                            );
                            println!("  State Changes: {}", shard_info.state_changes_count);
                        } else {
                            println!("  Chunk: None (empty shard)");
                        }
                        println!();
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
// ===========================================
