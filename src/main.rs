// src/main.rs (abridged)
use builder::topic::{parse_hex_0x_to_b32};
use builder::topic::backends::uniswap_fst::{load_uniswap_fst, UniswapFst};
use builder::topic::backends::binance_phf::BinancePhf;
use builder::topic::binance::BinanceChannelId;
use builder::topic::uniswap::UniswapEventId;
use builder::topic::router::TopicRouter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load snapshots for Uniswap (mmap)
    if let Some(dir) = std::env::var_os("TOPIC_MAP_DIR") {
        load_uniswap_fst(std::path::Path::new(&dir))?;
    }

    let router = TopicRouter { binance: BinancePhf, uniswap: UniswapFst };

    // Example lookups (ZERO allocations)
    if let Some(tid) = router.binance(BinanceChannelId::BookTicker, "ETHBTC") {
        println!("binance/book_ticker/ETHBTC => {tid}");
    }

    let pool_hex = "0x0003be2d3d4202dff5766085e6c00742a32ef88ebabed380ab1ec4fbb416604d";
    let pool = parse_hex_0x_to_b32(pool_hex)?;
    if let Some(tid) = router.uniswap(UniswapEventId::Swap, &pool) {
        println!("uniswap/swap/{pool_hex} => {tid}");
    }

    Ok(())
}
