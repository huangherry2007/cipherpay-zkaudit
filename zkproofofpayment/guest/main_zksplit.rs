#![no_std]
use risc0_zkvm::guest::env;
mod merkle;
use merkle::Hash;

fn main() {
    // Read inputs
    let recipients: [Hash; 2] = env::read();
    let amounts: [u64; 2] = env::read();

    // 1. Check recipients are unique and nonzero
    let unique = recipients[0] != recipients[1];
    let nonzero = recipients[0] != [0u8; 32] && recipients[1] != [0u8; 32];

    // 2. Check sum(amounts) > 0
    let sum = amounts[0] + amounts[1];
    let sum_valid = sum > 0;

    let is_valid = unique && nonzero && sum_valid;

    env::commit(&is_valid);
} 