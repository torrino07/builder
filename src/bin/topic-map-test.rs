use fst::Map;
use std::fs;
use alloy::primitives::FixedBytes;

fn main() -> anyhow::Result<()> {
    // ✅ Load FST from disk into memory
    let bytes = fs::read("snapshots/topic.map.fst")?;
    let map = Map::new(bytes)?;

    // ✅ Binance symbol test (zero alloc)
    let binance_symbol = "ETHBTC";
    if let Some(id) = map.get(binance_symbol.as_bytes()) {
        println!("Binance symbol '{}' → Topic ID: {}", binance_symbol, id);
    } else {
        println!("Binance symbol '{}' not found.", binance_symbol);
    }

    // ✅ Uniswap pool ID test (zero alloc)
    let uniswap_pool_id = "0x0003be2d3d4202dff5766085e6c00742a32ef88ebabed380ab1ec4fbb416604d";
    if let Ok(pool) = uniswap_pool_id.parse::<FixedBytes<32>>() {
        let mut buf = [0u8; 66];
        buf[0] = b'0';
        buf[1] = b'x';
        const HEX: &[u8; 16] = b"0123456789abcdef";
        for i in 0..32 {
            let b = pool[i];
            buf[2 + i * 2] = HEX[(b >> 4) as usize];
            buf[3 + i * 2] = HEX[(b & 0x0f) as usize];
        }

        if let Some(id) = map.get(&buf) {
            println!("Uniswap pool '{}' → Topic ID: {}", uniswap_pool_id, id);
        } else {
            println!("Uniswap pool '{}' not found.", uniswap_pool_id);
        }
    } else {
        println!("Invalid Uniswap pool ID format.");
    }

    Ok(())
}
