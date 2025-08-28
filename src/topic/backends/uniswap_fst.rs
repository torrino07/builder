// src/topic/backends/uniswap_fst.rs
use fst::Map;
use once_cell::sync::OnceCell;
use std::fs::File;
use std::path::Path;

use crate::topic::id::UNISWAP_INTERVAL;
use crate::topic::uniswap::{UniswapEventId, UniswapMap};

// Global, read-only, memory-mapped maps
static UNI_SWAP: OnceCell<Map<File>> = OnceCell::new();
// ... add other events as needed

pub fn load_uniswap_fst(dir: &Path) -> anyhow::Result<()> {
    // Expect files like: uniswap.swap.fst
    let swap = File::open(dir.join("uniswap.swap.fst"))?;
    UNI_SWAP.set(Map::new(swap)?).ok();
    Ok(())
}

#[inline]
fn hex_0x_lower_noalloc(pool: &[u8; 32]) -> [u8; 66] {
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

pub struct UniswapFst;

impl UniswapMap for UniswapFst {
    #[inline]
    fn topic_id(&self, ev: UniswapEventId, pool_b32: &[u8; 32]) -> Option<u32> {
        let key = hex_0x_lower_noalloc(pool_b32); // [u8;66]
        let local = match ev {
            UniswapEventId::Swap => UNI_SWAP.get()?.get(&key).map(|v| v as u32)?,
            // add other events...
        };
        Some(UNISWAP_INTERVAL.base + local)
    }
}
