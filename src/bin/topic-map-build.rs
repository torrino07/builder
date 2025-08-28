use std::{env, fs, path::Path};
use fst::MapBuilder;
use serde_json::Value;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: topic-map-build <binance.json> <uniswap.json> <out_dir>");
        std::process::exit(1);
    }

    let _bin_data = fs::read_to_string(&args[1])?;
    let uni_data = fs::read_to_string(&args[2])?;
    let out_dir = Path::new(&args[3]);
    fs::create_dir_all(out_dir)?;

    // Parse Uniswap JSON generically and collect all 0x + 64-hex strings
    let uni_json: Value = serde_json::from_str(&uni_data)?;
    let mut set: std::collections::HashSet<[u8; 32]> = std::collections::HashSet::new();
    collect_b32_hex_strings(&uni_json, &mut set);

    // Deterministic local IDs: sort, then index 0..N
    let mut pools: Vec<[u8; 32]> = set.into_iter().collect();
    pools.sort_unstable();

    let mut pairs: Vec<(Vec<u8>, u64)> = pools
        .into_iter()
        .enumerate()
        .map(|(i, p)| (b32_to_hex0x(p).to_vec(), i as u64))
        .collect();

    // FST requires sorted keys
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Write uniswap.swap.fst (start with Swap; extend later for Mint/Burn/etc.)
    let swap_path = out_dir.join("uniswap.swap.fst");
    let file = std::fs::File::create(&swap_path)?;
    let mut builder = MapBuilder::new(file)?;
    for (k, v) in &pairs {
        builder.insert(k, *v)?;
    }
    builder.finish()?;

    // Optional: simple manifest
    fs::write(
        out_dir.join("manifest.json"),
        serde_json::json!({
            "version": 1,
            "uniswap": { "swap_count": pairs.len() }
        })
        .to_string(),
    )?;

    eprintln!("Wrote {} ({} entries)", swap_path.display(), pairs.len());
    Ok(())
}

fn collect_b32_hex_strings(
    v: &serde_json::Value,
    out: &mut std::collections::HashSet<[u8; 32]>,
) {
    match v {
        Value::String(s) => {
            if looks_like_0x32bytes(s) {
                if let Ok(b) = parse_hex_0x_to_b32(s) {
                    out.insert(b);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                collect_b32_hex_strings(item, out);
            }
        }
        Value::Object(map) => {
            for val in map.values() {
                collect_b32_hex_strings(val, out);
            }
        }
        _ => {}
    }
}

#[inline]
fn looks_like_0x32bytes(s: &str) -> bool {
    if s.len() != 66 || !s.starts_with("0x") {
        return false;
    }
    s.as_bytes()[2..].iter().all(|c| {
        (b'0'..=b'9').contains(c) || (b'a'..=b'f').contains(c) || (b'A'..=b'F').contains(c)
    })
}

#[inline]
fn parse_hex_0x_to_b32(s: &str) -> anyhow::Result<[u8; 32]> {
    if !looks_like_0x32bytes(s) {
        anyhow::bail!("expected 0x + 64 hex bytes, got {}", s);
    }
    let bytes = hex::decode(&s[2..])?;
    if bytes.len() != 32 {
        anyhow::bail!("decoded length {}, expected 32", bytes.len());
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

#[inline]
fn b32_to_hex0x(pool: [u8; 32]) -> [u8; 66] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = [0u8; 66];
    out[0] = b'0';
    out[1] = b'x';
    for i in 0..32 {
        let b = pool[i];
        out[2 + i * 2] = HEX[(b >> 4) as usize];
        out[3 + i * 2] = HEX[(b & 0x0f) as usize];
    }
    out
}
