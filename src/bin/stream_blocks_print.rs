use std::env;
use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder, SSE};
use near_primitives::types::BlockHeight;
use serde::Deserialize;
use tokio_stream::StreamExt;
// ===========================================
const DEFAULT_NEAR_STREAM_URL: &str = "http://localhost:8080";
// ===========================================

#[derive(Debug, Deserialize)]
struct BlockEvent {
    block: BlockData,
}

#[derive(Debug, Deserialize)]
struct BlockData {
    header: BlockHeader,
    chunks: Vec<ChunkInfo>,
    #[serde(default)]
    author: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BlockHeader {
    height: BlockHeight,
    hash: String,
    prev_hash: String,
}

#[derive(Debug, Deserialize)]
struct ChunkInfo {
    shard_id: u64,
}

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

                    let block: BlockEvent = match serde_json::from_str(&data) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Failed to parse BlockView: {e}");
                            continue;
                        }
                    };

                    println!("===============================");
                    println!("Block #{}", block.block.header.height);
                    println!("Author: {}", block.block.author.as_deref().unwrap_or("unknown"));
                    println!("Hash: {}", block.block.header.hash);
                    println!("Prev: {}", block.block.header.prev_hash);
                    println!("Chunks: {}", block.block.chunks.len());
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
