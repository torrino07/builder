pub mod utils;
use std::{env, fs, path::Path};
use fst::MapBuilder;
use serde_json::Value;
use std::collections::HashSet;
use utils::{
    looks_like_0x32bytes,
    parse_hex_0x_to_b32
};

trait SourceParser {
    fn parse(&self, data: &str) -> anyhow::Result<Vec<Vec<u8>>>;
}

struct BinanceParser;
struct UniswapParser;

impl SourceParser for BinanceParser {
    fn parse(&self, data: &str) -> anyhow::Result<Vec<Vec<u8>>> {
        let json: Value = serde_json::from_str(data)?;
        let symbols = json.as_object()
            .ok_or_else(|| anyhow::anyhow!("Expected JSON object"))?
            .keys()
            .map(|k| k.as_bytes().to_vec())
            .collect();
        Ok(symbols)
    }
}

impl SourceParser for UniswapParser {
    fn parse(&self, data: &str) -> anyhow::Result<Vec<Vec<u8>>> {
        let json: Value = serde_json::from_str(data)?;
        let mut ids = std::collections::HashSet::<[u8; 32]>::new();
        collect_b32_hex_strings(&json, &mut ids);

        // Sort for stable ID assignment within source (like before)
        let mut pools: Vec<[u8; 32]> = ids.into_iter().collect();
        pools.sort_unstable();

        // IMPORTANT: store raw 32 bytes (no "0x", no hex encoding)
        Ok(pools.into_iter().map(|p| p.to_vec()).collect())
    }
}


struct Source<'a> {
    name: &'a str,
    parser: Box<dyn SourceParser>,
    path: &'a str,
    id_start: u64,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: topic-map-build <out_dir> <source1.json> <source2.json> ...");
        std::process::exit(1);
    }

    let out_dir = Path::new(&args[1]);
    fs::create_dir_all(out_dir)?;

    // Define sources with ID ranges
    let sources = vec![
        Source {
            name: "binance",
            parser: Box::new(BinanceParser),
            path: &args[2],
            id_start: 0,
        },
        Source {
            name: "uniswap",
            parser: Box::new(UniswapParser),
            path: &args[3],
            id_start: 10_000,
        },
    ];

    let mut all_pairs = vec![];
    let mut manifest = serde_json::Map::new();

    for source in sources {
        let data = fs::read_to_string(source.path)?;
        let entries = source.parser.parse(&data)?;
        let pairs: Vec<_> = entries
            .into_iter()
            .enumerate()
            .map(|(i, k)| (k, source.id_start + i as u64))
            .collect();
        manifest.insert(
            source.name.to_string(),
            serde_json::json!({ "count": pairs.len() }),
        );
        all_pairs.extend(pairs);
    }

    all_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let swap_path = out_dir.join("topic.map.fst");
    let file = std::fs::File::create(&swap_path)?;
    let mut builder = MapBuilder::new(file)?;
    for (k, v) in &all_pairs {
        builder.insert(k, *v)?;
    }
    builder.finish()?;

    manifest.insert("version".to_string(), serde_json::json!(1));
    fs::write(out_dir.join("manifest.json"), serde_json::to_string_pretty(&manifest)?)?;

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