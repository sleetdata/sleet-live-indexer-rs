use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::type_live::LIVE_BLOCK_EVENT;
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
    println!("Listening for new account creations...\n");

    let client = ClientBuilder::for_url(&url)?.build();
    let mut stream = client.stream();

    while let Some(event) = stream.next().await {
        match event {
            Ok(SSE::Event(ev)) => {
                let event_type = ev.event_type.as_str();

                if event_type == "block" {
                    let data = ev.data;

                    let block_event: LIVE_BLOCK_EVENT = match serde_json::from_str(&data) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Failed to parse BlockView: {e}");
                            continue;
                        }
                    };

                    let block_height = block_event.block.header.height;

                    // Iterate through shards to find account creation receipts
                    for shard in &block_event.shards {
                        // Receipts are inside shard.chunk.receipts
                        if let Some(chunk) = shard.get("chunk").and_then(|c| c.as_object()) {
                            // Check receipts for CreateAccount actions
                            if let Some(receipts) = chunk.get("receipts").and_then(|r| r.as_array()) {
                                for receipt in receipts {
                                    // Receipt structure: {predecessor_id, receiver_id, receipt_id, receipt: {Action: {actions: [...]}}}
                                    if let Some(receipt_obj) = receipt.get("receipt").and_then(|r| r.as_object()) {
                                        if let Some(action_receipt) = receipt_obj.get("Action") {
                                            if let Some(actions) = action_receipt.get("actions").and_then(|a| a.as_array()) {
                                                for action in actions {
                                                    if action.get("CreateAccount").is_some() {
                                                        let receiver_id = receipt
                                                            .get("receiver_id")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("unknown");

                                                        let predecessor_id = receipt
                                                            .get("predecessor_id")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("unknown");

                                                        let receipt_id = receipt
                                                            .get("receipt_id")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("unknown");

                                                        println!("===============================");
                                                        println!("🎉 New Account Created!");
                                                        println!("  Account:     {}", receiver_id);
                                                        println!("  Created by:  {}", predecessor_id);
                                                        println!("  Receipt ID:  {}", receipt_id);
                                                        println!("  Block:       {}", block_height);
                                                        println!("===============================");
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Also check transactions for CreateAccount actions
                            if let Some(txs) = chunk.get("transactions").and_then(|t| t.as_array()) {
                                for tx in txs {
                                    if let Some(tx_data) = tx.get("transaction").and_then(|t| t.as_object()) {
                                        if let Some(actions) = tx_data.get("actions").and_then(|a| a.as_array()) {
                                            for action in actions {
                                                if action.get("CreateAccount").is_some() {
                                                    let signer_id = tx_data
                                                        .get("signer_id")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("unknown");

                                                    let receiver_id = tx_data
                                                        .get("receiver_id")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("unknown");

                                                    let tx_hash = tx_data
                                                        .get("hash")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("unknown");

                                                    println!("===============================");
                                                    println!("🎉 New Account Created (Transaction)!");
                                                    println!("  Account:     {}", receiver_id);
                                                    println!("  Created by:  {}", signer_id);
                                                    println!("  TX Hash:     {}", tx_hash);
                                                    println!("  Block:       {}", block_height);
                                                    println!("===============================");
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
