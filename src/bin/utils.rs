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