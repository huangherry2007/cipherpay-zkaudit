#![no_std]
use risc0_zkvm::guest::env;
mod merkle;
use merkle::Hash;

fn main() {
    // Read inputs
    let condition_type: u8 = env::read();
    let value: u64 = env::read();

    // 1. Check condition_type is allowed (0 or 1)
    let is_type_valid = condition_type == 0 || condition_type == 1;

    // 2. Check value > 0
    let is_value_valid = value > 0;

    let is_valid = is_type_valid && is_value_valid;

    env::commit(&is_valid);
} 