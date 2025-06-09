#![no_std]
use poseidon::{Poseidon, Fr};

pub const DEPTH: usize = 32;
pub type Hash = [u8; 32];

pub fn verify_merkle_proof(leaf: Hash, root: Hash, path_elements: [Hash; DEPTH], path_indices: [u8; DEPTH]) -> bool {
    let mut current = leaf;
    for i in 0..DEPTH {
        let sibling = path_elements[i];
        current = if path_indices[i] == 0 {
            poseidon_hash2(current, sibling)
        } else {
            poseidon_hash2(sibling, current)
        };
    }
    current == root
}

pub fn poseidon_hash2(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let a_fr = Fr::from_bytes(&a).unwrap();
    let b_fr = Fr::from_bytes(&b).unwrap();
    let mut poseidon = Poseidon::new();
    poseidon.update(&[a_fr, b_fr]);
    let hash_fr = poseidon.finalize();
    hash_fr.to_bytes()
} 