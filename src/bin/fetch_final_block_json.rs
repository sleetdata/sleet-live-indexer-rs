use anyhow::{Context, Result};
use sleet_live_indexer_rs::types::NeardataBlockResponse;
use std::fs;
use std::path::Path;
// ===========================================
const NEARDATA_URL: &str = "https://mainnet.neardata.xyz/v0/last_block/final";
const TEMP_DIR: &str = "./temp";
const OUTPUT_FILE: &str = "final_block.json";
// ===========================================
#[tokio::main]
async fn main() -> Result<()> {
    // Create temp directory if it doesn't exist
    fs::create_dir_all(TEMP_DIR)
        .with_context(|| format!("Failed to create directory: {}", TEMP_DIR))?;

    println!("Fetching final block from: {}", NEARDATA_URL);

    // Fetch the block data
    let client = reqwest::Client::new();
    let res = client
        .get(NEARDATA_URL)
        .send()
        .await
        .with_context(|| "Failed to send HTTP request")?;

    if !res.status().is_success() {
        anyhow::bail!("Failed to fetch: {} {}", res.status(), res.status().canonical_reason().unwrap_or(""));
    }

    let json: serde_json::Value = res
        .json()
        .await
        .with_context(|| "Failed to parse JSON response")?;

    // Validate and parse into our type
    let validated: NeardataBlockResponse = serde_json::from_value(json.clone())
        .with_context(|| "Failed to validate block data against NeardataBlockResponse schema")?;

    println!("===========================================");
    println!("✓ Validation successful!");
    println!("  Block height: {}", validated.height());
    println!("  Author: {}", validated.author());
    println!("  Hash: {}", validated.hash());
    println!("  Shards: {}", validated.shard_count());
    println!("===========================================");

    // Write raw JSON to file
    let output_path = Path::new(TEMP_DIR).join(OUTPUT_FILE);
    let json_str = serde_json::to_string_pretty(&json)?;
    fs::write(&output_path, &json_str)
        .with_context(|| format!("Failed to write JSON to: {}", output_path.display()))?;

    println!("\n✓ Raw JSON saved to: {}", output_path.display());

    Ok(())
}
// ===========================================
