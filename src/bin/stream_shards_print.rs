use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::types::neardata_block_response_interface;
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

                    let block_event: neardata_block_response_interface = match serde_json::from_str(&data) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Failed to parse block: {e}");
                            continue;
                        }
                    };

                    println!("===============================");
                    println!("Block #{}", block_event.height());
                    println!("Author: {}", block_event.author());
                    println!("Hash: {}", block_event.hash());
                    println!("Shards: {}", block_event.shard_count());
                    println!("===============================");

                    for shard in &block_event.shards {
                        println!("--- Shard #{} ---", shard.shard_id);

                        let chunk = &shard.chunk;
                        println!("  Chunk Hash: {}", chunk.header.chunk_hash);
                        println!("  Height Created: {}", chunk.header.height_created);
                        println!("  Height Included: {}", chunk.header.height_included);
                        println!("  Gas Used: {}", chunk.header.gas_used);
                        println!("  Gas Limit: {}", chunk.header.gas_limit);
                        println!("  Balance Burnt: {}", chunk.header.balance_burnt);
                        println!("  Validator Reward: {}", chunk.header.validator_reward);
                        println!("  Rent Paid: {}", chunk.header.rent_paid);
                        println!("  Tx Root: {}", chunk.header.tx_root);
                        println!("  Outcome Root: {}", chunk.header.outcome_root);
                        println!("  Outgoing Receipts Root: {}", chunk.header.outgoing_receipts_root);
                        println!("  Prev State Root: {}", chunk.header.prev_state_root);
                        println!("  Encoded Length: {}", chunk.header.encoded_length);
                        println!("  Signature: {}", chunk.header.signature);
                        println!("  Transactions: {}", chunk.transactions.len());

                        println!("  Receipt Execution Outcomes: {}", shard.receipt_execution_outcomes.len());
                        println!("  State Changes: {}", shard.state_changes.len());
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
