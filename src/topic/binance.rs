// src/topic/binance.rs
pub enum BinanceChannelId {
    BookTicker = 0,
    // Add more channels...
}
pub trait BinanceMap {
    fn topic_id(&self, ch: BinanceChannelId, symbol: &str) -> Option<u32>;
}
