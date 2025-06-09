#![no_std]
use risc0_zkvm::guest::env;
mod merkle;
use merkle::{verify_merkle_proof, poseidon_hash2, Hash, DEPTH};

fn main() {
    // Read inputs
    let in_amount: u64 = env::read();
    let in_nullifier: Hash = env::read();
    let in_secret: Hash = env::read();
    let in_path_elements: [Hash; DEPTH] = env::read();
    let in_path_indices: [u8; DEPTH] = env::read();
    let out_commitment: Hash = env::read();
    let merkle_root: Hash = env::read();
    let recipient_pubkey: Hash = env::read();

    // 1. Check in_amount > 0
    let is_amount_valid = in_amount > 0;

    // 2. Compute in_commitment = Poseidon(in_amount, in_secret)
    let in_amount_bytes = in_amount.to_be_bytes();
    let in_commitment = poseidon_hash2_amount(in_amount_bytes, in_secret);

    // 3. Verify Merkle path for in_commitment → merkle_root
    let merkle_valid = verify_merkle_proof(in_commitment, merkle_root, in_path_elements, in_path_indices);

    // 4. Compute out_nullifier = Poseidon(in_nullifier, in_secret)
    let out_nullifier = poseidon_hash2(in_nullifier, in_secret);

    // 5. Compute out_commitment = Poseidon(in_amount, recipient_pubkey)
    let out_commitment_calc = poseidon_hash2_amount(in_amount_bytes, recipient_pubkey);

    // 6. Assert out_commitment matches public input
    let commitment_valid = out_commitment == out_commitment_calc;

    // 7. Set final validity
    let is_valid = is_amount_valid && merkle_valid && commitment_valid;

    // Commit out_nullifier and validity to the journal
    env::commit(&out_nullifier);
    env::commit(&is_valid);
}

// Helper for Poseidon hash of (u64, Hash)
fn poseidon_hash2_amount(a: [u8; 8], b: [u8; 32]) -> [u8; 32] {
    let mut a_padded = [0u8; 32];
    a_padded[24..].copy_from_slice(&a);
    poseidon_hash2(a_padded, b)
} 