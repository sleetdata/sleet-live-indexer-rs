use anyhow::{Context, Result};
use sleet_live_indexer_rs::fun::fetch_final_block_fun::fetch_final_block_fun;
use std::fs;
use std::path::Path;
// ===========================================
const TEMP_DIR: &str = "./temp";
const OUTPUT_FILE: &str = "final_block.json";
// ===========================================
#[tokio::main]
async fn main() -> Result<()> {
    // Create temp directory if it doesn't exist
    fs::create_dir_all(TEMP_DIR)
        .with_context(|| format!("Failed to create directory: {}", TEMP_DIR))?;

    // Fetch the block data using reusable function
    let validated = fetch_final_block_fun().await?;

    println!("===========================================");
    println!("✓ Validation successful!");
    println!("  Block height: {}", validated.height());
    println!("  Author: {}", validated.author());
    println!("  Hash: {}", validated.hash());
    println!("  Shards: {}", validated.shard_count());
    println!("===========================================");

    // Write raw JSON to file (fetch raw JSON separately for saving)
    let client = reqwest::Client::new();
    let res = client
        .get("https://mainnet.neardata.xyz/v0/last_block/final")
        .send()
        .await?;
    let json: serde_json::Value = res.json().await?;

    let output_path = Path::new(TEMP_DIR).join(OUTPUT_FILE);
    let json_str = serde_json::to_string_pretty(&json)?;
    fs::write(&output_path, &json_str)
        .with_context(|| format!("Failed to write JSON to: {}", output_path.display()))?;

    println!("\n✓ Raw JSON saved to: {}", output_path.display());

    Ok(())
}
// ===========================================
