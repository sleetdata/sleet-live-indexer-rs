use anyhow::{Context, Result};
use crate::types::neardata_block_response_interface;
// ===========================================
const NEARDATA_URL: &str = "https://mainnet.neardata.xyz/v0/last_block/final";
// ===========================================

pub async fn fetch_final_block_fun() -> Result<neardata_block_response_interface> {
    println!("Fetching final block from: {}", NEARDATA_URL);

    let client = reqwest::Client::new();
    let res = client
        .get(NEARDATA_URL)
        .send()
        .await
        .with_context(|| "Failed to send HTTP request")?;

    if !res.status().is_success() {
        anyhow::bail!(
            "Failed to fetch: {} {}",
            res.status(),
            res.status().canonical_reason().unwrap_or("")
        );
    }

    let json: serde_json::Value = res
        .json()
        .await
        .with_context(|| "Failed to parse JSON response")?;

    let validated: neardata_block_response_interface = serde_json::from_value(json.clone())
        .with_context(
            || "Failed to validate block data against neardata_block_response_interface schema",
        )?;

    Ok(validated)
}
// ===========================================
