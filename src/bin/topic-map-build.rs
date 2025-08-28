use std::{env, fs, path::Path};
use fst::MapBuilder;
use serde_json::Value;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: topic-map-build <binance.json> <uniswap.json> <out_dir>");
        std::process::exit(1);
    }

    let bin_data = fs::read_to_string(&args[1])?;
    let uni_data = fs::read_to_string(&args[2])?;
    let out_dir = Path::new(&args[3]);
    fs::create_dir_all(out_dir)?;

    // Parse Binance
    let bin_json: Value = serde_json::from_str(&bin_data)?;
    let bin_symbols: Vec<String> = bin_json.as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();

    // Assign Binance IDs: 0..9999
    let bin_pairs: Vec<(Vec<u8>, u64)> = bin_symbols
        .iter()
        .enumerate()
        .map(|(i, sym)| (sym.as_bytes().to_vec(), i as u64))
        .collect();

    // Parse Uniswap
    let uni_json: Value = serde_json::from_str(&uni_data)?;
    let mut uni_ids: HashSet<[u8; 32]> = HashSet::new();
    collect_b32_hex_strings(&uni_json, &mut uni_ids);

    // Assign Uniswap IDs: 10000..N
    let mut uni_pools: Vec<[u8; 32]> = uni_ids.into_iter().collect();
    uni_pools.sort_unstable();
    let uni_pairs: Vec<(Vec<u8>, u64)> = uni_pools
        .iter()
        .enumerate()
        .map(|(i, p)| (b32_to_hex0x(*p).to_vec(), 10000 + i as u64))
        .collect();

    // Merge and sort
    let mut all_pairs = vec![];
    all_pairs.extend(bin_pairs.iter().cloned());
    all_pairs.extend(uni_pairs.iter().cloned());
    all_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Write FST
    let swap_path = out_dir.join("topic.map.fst");
    let file = std::fs::File::create(&swap_path)?;
    let mut builder = MapBuilder::new(file)?;
    for (k, v) in &all_pairs {
        builder.insert(k, *v)?;
    }
    builder.finish()?;

    // Manifest
    fs::write(
        out_dir.join("manifest.json"),
        serde_json::json!({
            "version": 1,
            "binance": { "symbol_count": bin_pairs.len() },
            "uniswap": { "swap_count": uni_pairs.len() }
        })
        .to_string(),
    )?;

    eprintln!("Wrote {} ({} entries)", swap_path.display(), all_pairs.len());
    Ok(())
}

fn collect_b32_hex_strings(v: &Value, out: &mut HashSet<[u8; 32]>) {
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
            for (key, val) in map {
                collect_b32_hex_strings(&Value::String(key.clone()), out); // scan key
                collect_b32_hex_strings(val, out);                         // scan value
            }
        }
        _ => {}
    }
}

#[inline]
fn looks_like_0x32bytes(s: &str) -> bool {
    s.len() == 66 && s.starts_with("0x") &&
        s[2..].chars().all(|c| c.is_ascii_hexdigit())
}

#[inline]
fn parse_hex_0x_to_b32(s: &str) -> anyhow::Result<[u8; 32]> {
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
