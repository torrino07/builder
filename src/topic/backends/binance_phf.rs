// src/topic/backends/binance_phf.rs
use crate::topic::id::BINANCE_INTERVAL;
use crate::topic::binance::{BinanceMap, BinanceChannelId};

mod generated {
    include!(concat!(env!("OUT_DIR"), "/binance_generated.rs"));
}

pub struct BinancePhf;

impl BinanceMap for BinancePhf {
    #[inline]
    fn topic_id(&self, ch: BinanceChannelId, symbol: &str) -> Option<u32> {
        let local = match ch {
            BinanceChannelId::BookTicker => generated::BINANCE_BOOK_TICKER.get(symbol).copied(),
            // add match arms per-channel PHF map
        }?;
        Some(BINANCE_INTERVAL.base + local)
    }
}
