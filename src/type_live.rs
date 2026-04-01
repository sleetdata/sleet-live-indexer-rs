use near_primitives::views::{
    BlockView,
    // ChunkView,
};
// ===========================================
// Wrapper for the SSE event structure: {"block": BlockView, "shards": [...]}
#[derive(serde::Deserialize)]
pub struct LIVE_BLOCK_EVENT {
    pub block: BlockView,
    pub shards: Vec<serde_json::Value>,
}
// ===========================================
