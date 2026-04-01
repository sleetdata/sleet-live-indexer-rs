use near_primitives::views::{
    ActionView,
    BlockView,
    ChunkView,
    ReceiptView,
};
// ===========================================
// Wrapper for the SSE event structure: {"block": BlockView, "shards": [...]}
#[derive(serde::Deserialize)]
pub struct LIVE_BLOCK_EVENT {
    pub block: BlockView,
    pub shards: Vec<LIVE_SHARD_DATA>,
}
// ===========================================
// Shard data structure from NEAR Stream
#[derive(serde::Deserialize, Debug)]
pub struct LIVE_SHARD_DATA {
    pub chunk: Option<ChunkView>,
    pub receipts: Option<Vec<ReceiptView>>,
    pub transactions: Vec<serde_json::Value>,
}
// ===========================================

