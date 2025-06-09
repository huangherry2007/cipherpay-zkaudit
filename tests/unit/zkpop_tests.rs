use zkpop::{generate_proof, verify_proof, ProofError};

#[test]
fn test_proof_generation() {
    let input = "test_secret";
    let proof = generate_proof(input).unwrap();
    assert!(verify_proof(&proof, input).unwrap());
}

#[test]
fn test_proof_verification_failure() {
    let input = "test_secret";
    let wrong_input = "wrong_secret";
    let proof = generate_proof(input).unwrap();
    assert!(!verify_proof(&proof, wrong_input).unwrap());
}

#[test]
fn test_invalid_input() {
    let empty_input = "";
    assert!(matches!(generate_proof(empty_input), Err(ProofError::InvalidInput)));
}

#[test]
fn test_proof_serialization() {
    let input = "test_secret";
    let proof = generate_proof(input).unwrap();
    let serialized = serde_json::to_string(&proof).unwrap();
    let deserialized: zkpop::Proof = serde_json::from_str(&serialized).unwrap();
    assert!(verify_proof(&deserialized, input).unwrap());
}

#[test]
fn test_proof_size() {
    let input = "test_secret";
    let proof = generate_proof(input).unwrap();
    let serialized = serde_json::to_string(&proof).unwrap();
    // Ensure proof size is within reasonable bounds
    assert!(serialized.len() < 10000);
} 