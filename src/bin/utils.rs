#[inline]
pub fn b32_to_hex0x(pool: [u8; 32]) -> [u8; 66] {
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

#[inline]
pub fn looks_like_0x32bytes(s: &str) -> bool {
    s.len() == 66 && s.starts_with("0x") &&
        s[2..].chars().all(|c| c.is_ascii_hexdigit())
}

#[inline]
pub fn parse_hex_0x_to_b32(s: &str) -> anyhow::Result<[u8; 32]> {
    let bytes = hex::decode(&s[2..])?;
    if bytes.len() != 32 {
        anyhow::bail!("decoded length {}, expected 32", bytes.len());
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}