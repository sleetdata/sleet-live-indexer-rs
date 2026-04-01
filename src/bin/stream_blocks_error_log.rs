// Stream blocks and log any parse errors to a file for debugging
// Continues running even when errors occur
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::types::neardata_block_response_interface;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
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
                            log_error(ERROR_LOG_FILE, FAILED_BLOCK_DIR, &data, &format!("Raw JSON parse error: {}", e), total_blocks)?;
                            continue;
                        }
                    };

                    let block_height = raw
                        .get("block")
                        .and_then(|b| b.get("header"))
                        .and_then(|h| h.get("height"))
                        .and_then(|h| h.as_u64())
                        .unwrap_or(0);

                    // Try to parse as typed structure
                    match serde_json::from_str::<neardata_block_response_interface>(&data) {
                        Ok(block) => {
                            successful_blocks += 1;
                            println!("✓ Block #{} - {} shards", block.height(), block.shard_count());
                        }
                        Err(e) => {
                            failed_blocks += 1;
                            let error_msg = format!("Type parse error: {}", e);
                            log_error(ERROR_LOG_FILE, FAILED_BLOCK_DIR, &data, &error_msg, total_blocks)?;
                            eprintln!("✗ Block #{} - {}", block_height, error_msg);
                        }
                    }

                    // Print stats every 50 blocks
                    if total_blocks % 50 == 0 {
                        println!("\n=== Stats ===");
                        println!("Total: {} | Success: {} | Failed: {} | Error Rate: {:.2}%\n",
                            total_blocks,
                            successful_blocks,
                            failed_blocks,
                            (failed_blocks as f64 / total_blocks as f64) * 100.0
                        );
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

// Log error to file and save failed block JSON
fn log_error(
    log_file: &str,
    failed_dir: &str,
    block_data: &str,
    error_msg: &str,
    block_num: u64,
) -> Result<()> {
    // Append to error log
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    writeln!(log, "\n=== Block #{} ===", block_num)?;
    writeln!(log, "Error: {}", error_msg)?;
    writeln!(log, "Timestamp: {}", chrono::Utc::now().to_rfc3339())?;

    // Extract block height if possible
    if let Ok(raw) = serde_json::from_str::<serde_json::Value>(block_data) {
        if let Some(height) = raw
            .get("block")
            .and_then(|b| b.get("header"))
            .and_then(|h| h.get("height"))
        {
            writeln!(log, "Block Height: {}", height)?;

            // Save full JSON to file
            let filename = format!("{}/block_{}_error.json", failed_dir, height);
            std::fs::write(&filename, block_data)?;
            writeln!(log, "Saved to: {}", filename)?;
        }
    }

    writeln!(log, "----------------------------------------")?;

    Ok(())
}
// ==============================================
// copyright 2026 by sleet.near
