use crate::types::neardata_block_response_interface;
// ===========================================

pub struct BlockInfo {
    pub height: u64,
    pub author: String,
    pub hash: String,
    pub shard_count: usize,
}
// ===========================================

pub fn extract_block_info_fun(block: &neardata_block_response_interface) -> BlockInfo {
    BlockInfo {
        height: block.height(),
        author: block.author().to_string(),
        hash: block.hash().to_string(),
        shard_count: block.shard_count(),
    }
}
// ===========================================
