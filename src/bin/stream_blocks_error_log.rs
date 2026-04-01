// Stream blocks and log any parse errors to a file for debugging
// Continues running even when errors occur
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    log_error_fun::log_error_fun,
    parse_block_fun::parse_block_fun,
    print_block_stats_fun::print_block_stats_fun,
    save_failed_block_fun::save_failed_block_fun,
};
use std::env;
use tokio_stream::StreamExt;
// ==============================================
const DEFAULT_NEAR_STREAM_URL: &str = "http://localhost:8080";
const ERROR_LOG_FILE: &str = "./temp/parse_errors.log";
const FAILED_BLOCK_DIR: &str = "./temp/failed_blocks";
// ==============================================

#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("NEAR_STREAM_URL").unwrap_or_else(|_| DEFAULT_NEAR_STREAM_URL.to_string());

    println!("Connecting to NEAR Stream: {url}");
    println!("Error log: {}", ERROR_LOG_FILE);
    println!("Failed blocks saved to: {}\n", FAILED_BLOCK_DIR);

    // Create directories and files
    std::fs::create_dir_all("./temp")?;
    std::fs::create_dir_all(FAILED_BLOCK_DIR)?;

    let client = ClientBuilder::for_url(&url)?.build();
    let mut stream = client.stream();

    let mut total_blocks = 0;
    let mut successful_blocks = 0;
    let mut failed_blocks = 0;

    while let Some(event) = stream.next().await {
        match event {
            Ok(SSE::Event(ev)) => {
                let event_type = ev.event_type.as_str();

                if event_type == "block" {
                    let data = ev.data;
                    total_blocks += 1;

                    // Parse as raw JSON first to get block height
                    let raw: serde_json::Value = match serde_json::from_str(&data) {
                        Ok(v) => v,
                        Err(e) => {
                            failed_blocks += 1;
                            log_error_fun(ERROR_LOG_FILE, &data, &format!("Raw JSON parse error: {}", e), total_blocks)?;
                            continue;
                        }
                    };

                    let block_height = raw
                        .get("block")
                        .and_then(|b| b.get("header"))
                        .and_then(|h| h.get("height"))
                        .and_then(|h| h.as_u64())
                        .unwrap_or(0);

                    // Try to parse as typed structure using reusable function
                    match parse_block_fun(&data) {
                        Ok(_block) => {
                            successful_blocks += 1;
                            println!("✓ Block #{} - {} shards", block_height, _block.shard_count());
                        }
                        Err(e) => {
                            failed_blocks += 1;
                            let error_msg = format!("Type parse error: {}", e);
                            log_error_fun(ERROR_LOG_FILE, &data, &error_msg, total_blocks)?;
                            
                            // Save failed block using reusable function
                            if let Some(height) = raw
                                .get("block")
                                .and_then(|b| b.get("header"))
                                .and_then(|h| h.get("height"))
                                .and_then(|h| h.as_u64())
                            {
                                let filename = save_failed_block_fun(FAILED_BLOCK_DIR, height, &data)?;
                                eprintln!("✗ Block #{} - {} (saved to: {})", block_height, error_msg, filename);
                            } else {
                                eprintln!("✗ Block #{} - {}", block_height, error_msg);
                            }
                        }
                    }

                    // Print stats every 50 blocks using reusable function
                    if total_blocks % 50 == 0 {
                        print_block_stats_fun(total_blocks, successful_blocks, failed_blocks);
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
