#![no_std]
use risc0_zkvm::guest::env;
mod merkle;
use merkle::Hash;

fn main() {
    // Read inputs
    let stream_id: Hash = env::read();
    let total_amount: u64 = env::read();

    // 1. Check total_amount > 0
    let is_amount_valid = total_amount > 0;

    // 2. Check stream_id is not zero
    let is_id_valid = stream_id != [0u8; 32];

    let is_valid = is_amount_valid && is_id_valid;

    env::commit(&is_valid);
} 