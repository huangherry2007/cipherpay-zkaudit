#![no_std]
use risc0_zkvm::guest::env;
mod merkle;
use merkle::{verify_merkle_proof, Hash, DEPTH};

fn main() {
    let root: Hash = env::read();
    let leaf: Hash = env::read();
    let path_elements: [Hash; DEPTH] = env::read();
    let path_indices: [u8; DEPTH] = env::read();
    let valid = verify_merkle_proof(leaf, root, path_elements, path_indices);
    assert!(valid);
    env::commit(&root);
} 