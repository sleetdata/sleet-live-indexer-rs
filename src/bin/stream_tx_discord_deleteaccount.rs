// Stream DeleteAccount transactions for .near accounts only
// Saves to SQLite database and sends to Discord webhook
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    extract_block_info_fun::extract_block_info_fun,
    filter_account_types_fun::is_near_account_fun,
    filter_deleteaccount_actions_fun::filter_deleteaccount_actions_fun,
    format_delete_account_for_discord_fun::format_delete_account_for_discord_fun,
    parse_block_fun::parse_block_fun,
    send_discord_webhook_fun::send_discord_webhook_fun,
    sqlite_database_fun::{init_database_fun, save_deleted_account_fun},
};
use std::env;
use tokio_stream::StreamExt;
// ==============================================
const DEFAULT_NEAR_STREAM_URL: &str = "http://localhost:8080";
const ENV_WEBHOOK_URL: &str = "DISCORD_WEBHOOK_URL_DELETEACCOUNT";
// ==============================================

#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("NEAR_STREAM_URL").unwrap_or_else(|_| DEFAULT_NEAR_STREAM_URL.to_string());
    let webhook_url = env::var(ENV_WEBHOOK_URL)
        .expect(&format!("{} environment variable must be set", ENV_WEBHOOK_URL));

    println!("Connecting to NEAR Stream: {url}");
    println!("Filtering: DeleteAccount actions for .near accounts only");
    println!("Discord webhook: configured");
    println!("SQLite database: ./temp/indexer.db\n");

    // Initialize SQLite database
    let conn = init_database_fun(None)?;

    let client = ClientBuilder::for_url(&url)?.build();
    let mut stream = client.stream();

    let mut total_deleted = 0;
    let mut near_deleted = 0;
    let mut discord_sent = 0;
    let mut db_saved = 0;

    while let Some(event) = stream.next().await {
        match event {
            Ok(SSE::Event(ev)) => {
                let event_type = ev.event_type.as_str();

                if event_type == "block" {
                    let data = ev.data;

                    // Parse block using reusable function
                    let block = match parse_block_fun(&data) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Failed to parse block: {e}");
                            continue;
                        }
                    };

                    // Extract block info using reusable function
                    let block_info = extract_block_info_fun(&block);

                    // Filter DeleteAccount actions using reusable function
                    let delete_accounts = filter_deleteaccount_actions_fun(&block);

                    if !delete_accounts.is_empty() {
                        total_deleted += delete_accounts.len();

                        for da in &delete_accounts {
                            // Filter: only .near accounts (not .tg or implicit)
                            if !is_near_account_fun(&da.receiver_id) {
                                continue;
                            }

                            near_deleted += 1;

                            println!(
                                "Block #{} | Deleted: {} → Beneficiary: {}",
                                block_info.height,
                                da.receiver_id,
                                da.beneficiary_id
                            );

                            // Save to SQLite database
                            match save_deleted_account_fun(
                                &conn,
                                &da.tx_hash,
                                block_info.height,
                                da.shard_id,
                                &da.receiver_id,
                                &da.beneficiary_id,
                                &da.signer_id,
                                Some(&da.priority_fee.to_string()),
                            ) {
                                Ok(id) => {
                                    println!("  ✓ Saved to SQLite (id: {})", id);
                                    db_saved += 1;
                                }
                                Err(e) => {
                                    eprintln!("  ✗ Failed to save to SQLite: {e}");
                                }
                            }

                            // Format for Discord using reusable function
                            let embed = format_delete_account_for_discord_fun(
                                &da.receiver_id,
                                &da.beneficiary_id,
                                &da.signer_id,
                                &da.tx_hash,
                                block_info.height,
                                da.shard_id,
                                Some(&da.priority_fee.to_string()),
                            );

                            // Send to Discord using reusable function
                            match send_discord_webhook_fun(&webhook_url, embed).await {
                                Ok(_) => {
                                    println!("  ✓ Sent to Discord");
                                    discord_sent += 1;
                                }
                                Err(e) => {
                                    eprintln!("  ✗ Failed to send to Discord: {e}");
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

    println!("\n=== Final Stats ===");
    println!("Total DeleteAccount actions: {}", total_deleted);
    println!(".near accounts deleted: {}", near_deleted);
    println!("Saved to SQLite: {}", db_saved);
    println!("Sent to Discord: {}", discord_sent);

    Ok(())
}
// ==============================================
// copyright 2026 by sleet.near
