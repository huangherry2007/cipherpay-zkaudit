#![no_std]
use risc0_zkvm::guest::env;
mod merkle;
mod audit;
use merkle::Hash;
use audit::verify_audit_proof;

const CURRENT_TIME: u64 = 1234567890;

fn main() {
    let note_commitment: Hash = env::read();
    let view_key: Hash = env::read();
    let amount: u64 = env::read();
    let timestamp: u64 = env::read();
    let purpose: u64 = env::read();
    let audit_id: Hash = env::read();
    let merkle_root: Hash = env::read();
    let is_valid = verify_audit_proof(
        note_commitment,
        view_key,
        amount,
        timestamp,
        purpose,
        audit_id,
        merkle_root,
        CURRENT_TIME,
    );
    env::commit(&is_valid);
    env::commit(&merkle_root);
} 