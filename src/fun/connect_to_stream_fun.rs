use anyhow::Result;
use eventsource_client::{Client as _, ClientBuilder};
use tokio_stream::Stream;
// ===========================================
const DEFAULT_NEAR_STREAM_URL: &str = "http://localhost:8080";
// ===========================================

pub fn connect_to_stream_fun(url: Option<&str>) -> Result<impl Stream<Item = Result<eventsource_client::SSE, eventsource_client::Error>>> {
    let stream_url = url.unwrap_or(DEFAULT_NEAR_STREAM_URL);

    println!("Connecting to NEAR Stream: {}", stream_url);

    let client = ClientBuilder::for_url(stream_url)?.build();
    let stream = client.stream();

    Ok(stream)
}
// ===========================================
