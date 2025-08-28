use phf::phf_map;
pub static BINANCE_BOOK_TICKER: phf::Map<&'static str, u32> = phf_map! {
    "ETHBTC" => 1,
};
