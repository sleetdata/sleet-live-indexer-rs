// ===========================================

pub fn print_block_stats_fun(total: u64, success: u64, failed: u64) {
    let error_rate = if total > 0 {
        (failed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("\n=== Stats ===");
    println!(
        "Total: {} | Success: {} | Failed: {} | Error Rate: {:.2}%\n",
        total, success, failed, error_rate
    );
}
// ===========================================
