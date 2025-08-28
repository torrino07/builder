// build.rs
use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let gen_bin = out_dir.join("binance_generated.rs");

    // TODO: load the real JSONs (path via env/cargo or predictable path)
    // let binance: BinanceDict = serde_json::from_str(&fs::read_to_string("binance.json")?).unwrap();
    // let bt_symbols: Vec<(String, u32)> = build_local_ids_for_channel(&binance, "book_ticker");

    let bt_symbols = vec![
        ("ETHBTC".to_string(), 1u32),
        // ...
    ];

    let mut s = String::new();
    s.push_str("use phf::phf_map;\n");
    s.push_str("pub static BINANCE_BOOK_TICKER: phf::Map<&'static str, u32> = phf_map! {\n");
    for (sym, id) in &bt_symbols {
        s.push_str(&format!("    {:?} => {},\n", sym, id));
    }
    s.push_str("};\n");

    fs::write(gen_bin, s).unwrap();

    println!("cargo:rerun-if-changed=path/to/binance.json");
}

