// src/topic/router.rs
use crate::topic::binance::{BinanceMap, BinanceChannelId};
use crate::topic::uniswap::{UniswapMap, UniswapEventId};

pub struct TopicRouter<B: BinanceMap, U: UniswapMap> {
    pub binance: B,
    pub uniswap: U,
    // later: coinbase, kraken, curve, etc.
}

impl<B: BinanceMap, U: UniswapMap> TopicRouter<B, U> {
    #[inline]
    pub fn binance(&self, ch: BinanceChannelId, symbol: &str) -> Option<u32> {
        self.binance.topic_id(ch, symbol)
    }
    #[inline]
    pub fn uniswap(&self, ev: UniswapEventId, pool: &[u8;32]) -> Option<u32> {
        self.uniswap.topic_id(ev, pool)
    }
}
