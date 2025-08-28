// src/topic/uniswap.rs
pub enum UniswapEventId {
    Swap = 0,
    // Mint, Burn, etc.
}
pub trait UniswapMap {
    fn topic_id(&self, ev: UniswapEventId, pool_b32: &[u8; 32]) -> Option<u32>;
}
