use zkpop::{generate_proof, verify_proof};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: zkverifier <input> <proof>");
        return;
    }
    let input = &args[1];
    let proof = &args[2];
    if verify_proof(proof, input) {
        println!("Proof is valid for input: {}", input);
    } else {
        println!("Invalid proof for input: {}", input);
    }
} 