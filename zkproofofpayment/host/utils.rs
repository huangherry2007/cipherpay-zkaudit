pub type Hash = [u8; 32];

pub fn hex_to_bytes32(hex: &str) -> Hash {
    let hex = hex.trim_start_matches("0x");
    let mut bytes = [0u8; 32];
    let hex_bytes = hex::decode(hex).unwrap();
    let offset = 32 - hex_bytes.len();
    bytes[offset..].copy_from_slice(&hex_bytes);
    bytes
}

pub fn print_result(label: &str, i: usize, valid: bool) {
    println!("[{}][Test {}] Proof valid: {}", label, i, valid);
} 