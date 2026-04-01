use near_primitives::views::{
    BlockView,
    // ChunkHeaderView,
    // BlockHeaderView,
};
// ===========================================
// Wrapper for the SSE event structure: {"block": BlockView, "shards": [...]}
#[derive(serde::Deserialize)]
pub struct LIVE_BLOCK_EVENT {
    pub block: BlockView,
}
// ===========================================
