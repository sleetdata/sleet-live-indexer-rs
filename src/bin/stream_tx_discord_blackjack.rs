// Stream blackjack transactions and send them to Discord webhook
// Requires DISCORD_WEBHOOK_URL_BLACKJACK environment variable
// ==============================================
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use sleet_live_indexer_rs::fun::{
    extract_block_info_fun::extract_block_info_fun,
    filter_by_receiver_fun::filter_by_receiver_fun,
    format_blackjack_tx_for_discord_fun::format_blackjack_tx_for_discord_fun,
    parse_block_fun::parse_block_fun,
    send_discord_webhook_fun::send_discord_webhook_fun,
};
use std::env;
use tokio_stream::StreamExt;
// ==============================================
const DEFAULT_NEAR_STREAM_URL: &str = "http://localhost:8080";
const TARGET_RECEIVER: &str = "blackjack-v2.warsofcards.near";
// ==============================================

#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("NEAR_STREAM_URL").unwrap_or_else(|_| DEFAULT_NEAR_STREAM_URL.to_string());
    let webhook_url = env::var("DISCORD_WEBHOOK_URL_BLACKJACK")
        .expect("DISCORD_WEBHOOK_URL_BLACKJACK environment variable must be set");

    println!("Connecting to NEAR Stream: {url}");
    println!("Filtering for receiver: {}", TARGET_RECEIVER);
    println!("Discord webhook: configured\n");

    let client = ClientBuilder::for_url(&url)?.build();
    let mut stream = client.stream();

    let mut tx_count = 0;

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

                    // Filter by receiver using reusable function
                    let receiver_txs = filter_by_receiver_fun(&block, TARGET_RECEIVER);

                    if !receiver_txs.is_empty() {
                        println!(
                            "Block #{} | {} tx(s) to {}",
                            block_info.height,
                            receiver_txs.len(),
                            TARGET_RECEIVER
                        );

                        // Send each transaction to Discord
                        for tx in &receiver_txs {
                            tx_count += 1;

                            // Format for Discord using reusable function
                            let embed = format_blackjack_tx_for_discord_fun(
                                tx,
                                block_info.height,
                                TARGET_RECEIVER,
                            );

                            // Send to Discord using reusable function
                            match send_discord_webhook_fun(&webhook_url, embed).await {
                                Ok(_) => {
                                    println!("  ✓ Tx {} sent to Discord", tx.tx_hash);
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

    println!("\nTotal transactions sent to Discord: {}", tx_count);

    Ok(())
}
// ==============================================
// copyright 2026 by sleet.near
