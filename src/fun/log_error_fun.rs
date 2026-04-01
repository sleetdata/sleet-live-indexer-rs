use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
// ===========================================

pub fn log_error_fun(log_file: &str, _block_data: &str, error_msg: &str, block_num: u64) -> Result<()> {
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    writeln!(log, "\n=== Block #{} ===", block_num)?;
    writeln!(log, "Error: {}", error_msg)?;
    writeln!(log, "Timestamp: {}", chrono::Utc::now().to_rfc3339())?;

    Ok(())
}
// ===========================================
