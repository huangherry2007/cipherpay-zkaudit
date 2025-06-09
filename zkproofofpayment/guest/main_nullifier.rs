#![no_std]
use risc0_zkvm::guest::env;
mod merkle;
use merkle::{poseidon_hash2, Hash};

fn main() {
    // Read inputs
    let note_commitment: Hash = env::read();
    let secret: Hash = env::read();

    // 1. Check inputs are nonzero
    let is_valid = note_commitment != [0u8; 32] && secret != [0u8; 32];

    // 2. Compute nullifier = Poseidon(note_commitment, secret)
    let nullifier = poseidon_hash2(note_commitment, secret);

    // Commit nullifier and validity to the journal
    env::commit(&nullifier);
    env::commit(&is_valid);
} 