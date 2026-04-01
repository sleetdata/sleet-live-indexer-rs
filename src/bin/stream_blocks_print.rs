use std::env;
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use tokio_stream::StreamExt;
use near_primitives::views::BlockView;
// ===========================================
const DEFAULT_NEAR_STREAM_URL: &str = "http://localhost:8080";
// ===========================================
// main
#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("NEAR_STREAM_URL").unwrap_or_else(|_| DEFAULT_NEAR_STREAM_URL.to_string());

    println!("Connecting to NEAR Stream: {url}");

    let client = ClientBuilder::for_url(&url)?.build();
    let mut stream = client.stream();

    while let Some(event) = stream.next().await {
        match event {
            Ok(SSE::Event(ev)) => {
                let event_type = ev.event_type.as_str();

                if event_type == "block" {
                    let data = ev.data;

                    // Deserialize directly into NEAR’s official type
                    let block: BlockView = match serde_json::from_str(&data) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Failed to parse BlockView: {e}");
                            continue;
                        }
                    };

                    println!("===============================");
                    println!("Block #{}", block.header.height);
                    println!("Author: {}", block.author);
                    println!("Hash: {}", block.header.hash);
                    println!("Prev: {}", block.header.prev_hash);
                    println!("Chunks: {}", block.chunks.len());
                    println!("===============================");
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
// ===========================================
