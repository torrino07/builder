use fst::Map;
use std::fs;
use alloy::primitives::FixedBytes;

fn main() -> anyhow::Result<()> {
    // If you want to avoid copying the file into memory, use Map::from_path for mmap:
    // let map = Map::from_path("topic.map.fst")?;
    let bytes = fs::read("topic.map.fst")?;
    let map = Map::new(bytes)?;

    // ✅ Binance (ASCII key) — byte literal avoids allocation
    let binance_symbol = b"ETHBTC";
    if let Some(id) = map.get(binance_symbol) {
        println!("Binance 'ETHUSDT' → Topic ID: {id}");
    } else {
        println!("Binance 'ETHUSDT' not found.");
    }

    // ✅ Uniswap (raw 32-byte key)
    // IMPORTANT: This assumes your builder stored Uniswap keys as RAW 32 BYTES (no "0x", no hex)
    let pool_str = "0x000a193942d54b2c53c150653b377006504bcd2892846e45495a9a0af1f45e3e";

    // Parse the hex string once into FixedBytes<32>
    let pool: FixedBytes<32> = pool_str.parse()?; // errors if format/len invalid

    println!("{:?}", pool);
    println!("{:?}", pool.as_slice());

    // Avoid type inference ambiguity (E0283) by specifying K = &[u8]
    if let Some(id) = map.get(&pool.as_slice()) {
        println!("Uniswap '{pool_str}' → Topic ID: {id}");
    } else {
        println!("Uniswap '{pool_str}' not found.");
    }

    Ok(())
}
