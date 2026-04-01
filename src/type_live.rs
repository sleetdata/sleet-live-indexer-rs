use near_primitives::views::{
    BlockView, ChunkView, ExecutionOutcomeWithIdView, StateChangeValueView,
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
    pub receipt_execution_outcomes: Vec<ExecutionOutcomeWithIdView>,
    pub shard_id: u64,
    pub state_changes: Vec<StateChangeValueView>,
}
// ===========================================
