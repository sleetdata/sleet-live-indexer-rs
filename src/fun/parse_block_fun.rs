use anyhow::Result;
use crate::types::neardata_block_response_interface;
// ===========================================

pub fn parse_block_fun(data: &str) -> Result<neardata_block_response_interface> {
    let block: neardata_block_response_interface = serde_json::from_str(data)?;
    Ok(block)
}
// ===========================================
