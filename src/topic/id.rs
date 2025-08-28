// src/topic/id.rs
#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub base: u32,
    pub size: u32,
}
impl Interval {
    pub const fn new(base: u32, size: u32) -> Self { Self { base, size } }
}

// Example reservations (adjust to your real plan)
pub const BINANCE_INTERVAL: Interval = Interval::new(0, 10000);
pub const UNISWAP_INTERVAL: Interval = Interval::new(10001, 25000);
