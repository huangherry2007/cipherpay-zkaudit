#![no_std]
use poseidon::{Poseidon, Fr};
use crate::merkle::{poseidon_hash2, Hash};

pub fn verify_audit_proof(
    note_commitment: Hash,
    view_key: Hash,
    amount: u64,
    timestamp: u64,
    purpose: u64,
    audit_id: Hash,
    merkle_root: Hash,
    current_time: u64,
) -> bool {
    // 1. Verify view key matches commitment (Poseidon hash)
    let commitment_hash = poseidon_hash2(note_commitment, view_key);
    // 2. Check amount is positive
    let is_amount_valid = amount > 0;
    // 3. Check timestamp is not in the future
    let is_timestamp_valid = timestamp <= current_time;
    // 4. Generate audit hash (auditId) by hashing commitmentHash and amount
    let amount_bytes = amount.to_be_bytes();
    let amount_hash = poseidon_hash2_amount(commitment_hash, amount_bytes);
    // 5. Assert audit hash matches public auditId
    let audit_id_valid = amount_hash == audit_id;
    // 6. Set final validity
    is_amount_valid && is_timestamp_valid && audit_id_valid
}

// Overload for hashing a [u8; 32] and [u8; 8] (for amount)
pub fn poseidon_hash2_amount(a: [u8; 32], b: [u8; 8]) -> [u8; 32] {
    let a_fr = Fr::from_bytes(&a).unwrap();
    let mut b_padded = [0u8; 32];
    b_padded[24..].copy_from_slice(&b);
    let b_fr = Fr::from_bytes(&b_padded).unwrap();
    let mut poseidon = Poseidon::new();
    poseidon.update(&[a_fr, b_fr]);
    let hash_fr = poseidon.finalize();
    hash_fr.to_bytes()
} 