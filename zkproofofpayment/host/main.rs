mod test_vectors;
mod utils;
use test_vectors::*;
use utils::*;
use risc0_zkvm::{default_prover, ExecutorEnv, serde::to_vec};
use std::error::Error;

const DEPTH: usize = 32;

type Hash = [u8; 32];

fn hex_to_bytes32(hex: &str) -> Hash {
    let hex = hex.trim_start_matches("0x");
    let mut bytes = [0u8; 32];
    let hex_bytes = hex::decode(hex).unwrap();
    let offset = 32 - hex_bytes.len();
    bytes[offset..].copy_from_slice(&hex_bytes);
    bytes
}

fn run_merkle() -> Result<(), Box<dyn Error>> {
    for (i, tv) in merkle_test_vectors().into_iter().enumerate() {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&tv.root)?)
            .add_input(&to_vec(&tv.leaf)?)
            .add_input(&to_vec(&tv.path_elements)?)
            .add_input(&to_vec(&tv.path_indices)?)
            .build()?;
        let guest_path = "../guest/main_merkle";
        let prover = default_prover();
        let receipt = prover.prove(env, guest_path)?;
        let computed_root: Hash = receipt.journal.decode()?;
        println!("[Merkle][Test {}] Computed root in zkVM: 0x{}", i, hex::encode(computed_root));
        receipt.verify(guest_path)?;
        println!("[Merkle][Test {}] Proof verified!", i);
    }
    Ok(())
}

fn run_audit() -> Result<(), Box<dyn Error>> {
    for (i, tv) in audit_test_vectors().into_iter().enumerate() {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&tv.note_commitment)?)
            .add_input(&to_vec(&tv.view_key)?)
            .add_input(&to_vec(&tv.amount)?)
            .add_input(&to_vec(&tv.timestamp)?)
            .add_input(&to_vec(&tv.purpose)?)
            .add_input(&to_vec(&tv.audit_id)?)
            .add_input(&to_vec(&tv.merkle_root)?)
            .build()?;
        let guest_path = "../guest/main_audit";
        let prover = default_prover();
        let receipt = prover.prove(env, guest_path)?;
        let is_valid: bool = receipt.journal.decode()?;
        let returned_merkle_root: Hash = receipt.journal.decode()?;
        println!("[Audit][Test {}] Proof valid: {}", i, is_valid);
        println!("[Audit][Test {}] Merkle root: 0x{}", i, hex::encode(returned_merkle_root));
        receipt.verify(guest_path)?;
        println!("[Audit][Test {}] Proof verified!", i);
    }
    Ok(())
}

fn run_transfer() -> Result<(), Box<dyn Error>> {
    for (i, tv) in transfer_test_vectors().into_iter().enumerate() {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&tv.amount)?)
            .add_input(&to_vec(&tv.sender)?)
            .add_input(&to_vec(&tv.receiver)?)
            .build()?;
        let guest_path = "../guest/main_transfer";
        let prover = default_prover();
        let receipt = prover.prove(env, guest_path)?;
        let valid: bool = receipt.journal.decode()?;
        print_result("Transfer", i, valid);
        receipt.verify(guest_path)?;
    }
    Ok(())
}

fn run_withdraw() -> Result<(), Box<dyn Error>> {
    for (i, tv) in withdraw_test_vectors().into_iter().enumerate() {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&tv.amount)?)
            .add_input(&to_vec(&tv.recipient)?)
            .build()?;
        let guest_path = "../guest/main_withdraw";
        let prover = default_prover();
        let receipt = prover.prove(env, guest_path)?;
        let valid: bool = receipt.journal.decode()?;
        print_result("Withdraw", i, valid);
        receipt.verify(guest_path)?;
    }
    Ok(())
}

fn run_nullifier() -> Result<(), Box<dyn Error>> {
    for (i, tv) in nullifier_test_vectors().into_iter().enumerate() {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&tv.nullifier)?)
            .add_input(&to_vec(&tv.secret)?)
            .build()?;
        let guest_path = "../guest/main_nullifier";
        let prover = default_prover();
        let receipt = prover.prove(env, guest_path)?;
        let valid: bool = receipt.journal.decode()?;
        print_result("Nullifier", i, valid);
        receipt.verify(guest_path)?;
    }
    Ok(())
}

fn run_zkcondition() -> Result<(), Box<dyn Error>> {
    for (i, tv) in zkcondition_test_vectors().into_iter().enumerate() {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&tv.condition_type)?)
            .add_input(&to_vec(&tv.value)?)
            .build()?;
        let guest_path = "../guest/main_zkcondition";
        let prover = default_prover();
        let receipt = prover.prove(env, guest_path)?;
        let valid: bool = receipt.journal.decode()?;
        print_result("ZkCondition", i, valid);
        receipt.verify(guest_path)?;
    }
    Ok(())
}

fn run_zksplit() -> Result<(), Box<dyn Error>> {
    for (i, tv) in zksplit_test_vectors().into_iter().enumerate() {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&tv.recipients)?)
            .add_input(&to_vec(&tv.amounts)?)
            .build()?;
        let guest_path = "../guest/main_zksplit";
        let prover = default_prover();
        let receipt = prover.prove(env, guest_path)?;
        let valid: bool = receipt.journal.decode()?;
        print_result("ZkSplit", i, valid);
        receipt.verify(guest_path)?;
    }
    Ok(())
}

fn run_zkstream() -> Result<(), Box<dyn Error>> {
    for (i, tv) in zkstream_test_vectors().into_iter().enumerate() {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&tv.stream_id)?)
            .add_input(&to_vec(&tv.total_amount)?)
            .build()?;
        let guest_path = "../guest/main_zkstream";
        let prover = default_prover();
        let receipt = prover.prove(env, guest_path)?;
        let valid: bool = receipt.journal.decode()?;
        print_result("ZkStream", i, valid);
        receipt.verify(guest_path)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: host <merkle|audit|transfer|withdraw|nullifier|zkcondition|zksplit|zkstream>");
        return Ok(());
    }
    match args[1].as_str() {
        "merkle" => run_merkle(),
        "audit" => run_audit(),
        "transfer" => run_transfer(),
        "withdraw" => run_withdraw(),
        "nullifier" => run_nullifier(),
        "zkcondition" => run_zkcondition(),
        "zksplit" => run_zksplit(),
        "zkstream" => run_zkstream(),
        _ => {
            println!("Unknown mode: {}", args[1]);
            Ok(())
        }
    }
} 