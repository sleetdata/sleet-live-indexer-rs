use anyhow::Result;
use std::fs;
// ===========================================

pub fn save_failed_block_fun(failed_dir: &str, block_height: u64, block_data: &str) -> Result<String> {
    let filename = format!("{}/block_{}_error.json", failed_dir, block_height);
    fs::write(&filename, block_data)?;
    Ok(filename)
}
// ===========================================
