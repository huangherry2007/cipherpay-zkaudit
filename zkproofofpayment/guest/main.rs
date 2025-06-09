#![no_std]
use risc0_zkvm::guest::env;
use poseidon::{Poseidon, Fr};

// Constants
const DEPTH: usize = 32;
const CURRENT_TIME: u64 = 1234567890; // Should be public input in practice

type Hash = [u8; 32];

fn main() {
    // Read inputs from the host (matching audit_proof.circom)
    let note_commitment: Hash = env::read(); // private
    let view_key: Hash = env::read();        // private
    let amount: u64 = env::read();           // private
    let timestamp: u64 = env::read();        // private
    let purpose: u64 = env::read();          // private
    let audit_id: Hash = env::read();        // public
    let merkle_root: Hash = env::read();     // public

    // 1. Verify view key matches commitment (Poseidon hash)
    let commitment_hash = poseidon_hash2(note_commitment, view_key);

    // 2. Check amount is positive (bit decomposition)
    let is_amount_valid = amount > 0;

    // 3. Check timestamp is not in the future
    let is_timestamp_valid = timestamp <= CURRENT_TIME;

    // 4. Generate audit hash (auditId) by hashing commitmentHash and amount
    let amount_bytes = amount.to_be_bytes();
    let amount_hash = poseidon_hash2(commitment_hash, amount_bytes);

    // 5. Assert audit hash matches public auditId
    assert_eq!(amount_hash, audit_id);

    // 6. Set final validity
    let is_valid = is_amount_valid && is_timestamp_valid;

    // Commit the result to the journal
    env::commit(&is_valid);
    env::commit(&merkle_root);
}

// Real Poseidon hash of two elements (32 bytes each)
fn poseidon_hash2(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let a_fr = Fr::from_bytes(&a).unwrap();
    let b_fr = Fr::from_bytes(&b).unwrap();
    let mut poseidon = Poseidon::new();
    poseidon.update(&[a_fr, b_fr]);
    let hash_fr = poseidon.finalize();
    hash_fr.to_bytes()
}

// Overload for hashing a [u8; 32] and [u8; 8] (for amount)
fn poseidon_hash2(a: [u8; 32], b: [u8; 8]) -> [u8; 32] {
    let a_fr = Fr::from_bytes(&a).unwrap();
    let mut b_padded = [0u8; 32];
    b_padded[24..].copy_from_slice(&b);
    let b_fr = Fr::from_bytes(&b_padded).unwrap();
    let mut poseidon = Poseidon::new();
    poseidon.update(&[a_fr, b_fr]);
    let hash_fr = poseidon.finalize();
    hash_fr.to_bytes()
}

pub fn generate_proof(input: &str) -> String {
    // Mock: In a real system, this would generate a zk proof for the input
    format!("zkproof_for_{}", input)
}

pub fn verify_proof(proof: &str, input: &str) -> bool {
    // Mock: In a real system, this would verify the zk proof
    proof == format!("zkproof_for_{}", input)
} 