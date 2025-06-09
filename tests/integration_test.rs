use zkpop::{generate_proof, verify_proof};
use zkscanner::{Note, scan_notes};
use std::collections::HashMap;

#[test]
fn test_basic_integration() {
    // Test basic proof generation and verification
    let input = "test_secret";
    let proof = generate_proof(input).unwrap();
    assert!(verify_proof(&proof, input).unwrap());
}

#[test]
fn test_zkpop_and_zkscanner_integration() {
    // Test integration between zkPoP and note scanning
    let input = "my_secret";
    let proof = generate_proof(input).unwrap();
    assert!(verify_proof(&proof, input).unwrap());

    let notes = vec![
        Note { id: 1, view_key: "vk1".to_string(), data: "note1".to_string() },
        Note { id: 2, view_key: "vk2".to_string(), data: "note2".to_string() },
        Note { id: 3, view_key: "vk1".to_string(), data: "note3".to_string() },
    ];
    let found = scan_notes(&notes, "vk1").unwrap();
    assert_eq!(found.len(), 2);
    assert_eq!(found[0].data, "note1");
    assert_eq!(found[1].data, "note3");
}

#[test]
fn test_complex_transaction_flow() {
    // Test a more complex transaction flow with multiple proofs and notes
    let transactions = vec![
        ("tx1", "vk1", "100"),
        ("tx2", "vk2", "200"),
        ("tx3", "vk1", "300"),
    ];

    let mut proofs = HashMap::new();
    let mut notes = Vec::new();

    // Generate proofs and notes for each transaction
    for (i, (tx_id, view_key, amount)) in transactions.iter().enumerate() {
        let proof = generate_proof(&format!("{}_{}", tx_id, amount)).unwrap();
        proofs.insert(tx_id, proof);

        notes.push(Note {
            id: i as u64,
            view_key: view_key.to_string(),
            data: format!("Transaction {}: {}", tx_id, amount),
        });
    }

    // Verify all proofs
    for (tx_id, (_, _, amount)) in transactions.iter().enumerate() {
        let proof = proofs.get(&transactions[tx_id].0).unwrap();
        assert!(verify_proof(proof, &format!("{}_{}", transactions[tx_id].0, amount)).unwrap());
    }

    // Scan notes for vk1
    let found = scan_notes(&notes, "vk1").unwrap();
    assert_eq!(found.len(), 2);
    assert!(found.iter().any(|n| n.data.contains("tx1")));
    assert!(found.iter().any(|n| n.data.contains("tx3")));
}

#[test]
fn test_error_handling_integration() {
    // Test error handling across components
    let invalid_proof = generate_proof("").unwrap_err();
    assert!(matches!(invalid_proof, zkpop::ProofError::InvalidInput));

    let notes = vec![
        Note { id: 1, view_key: "vk1".to_string(), data: "note1".to_string() },
        Note { id: 1, view_key: "vk1".to_string(), data: "note2".to_string() },
    ];
    let scan_error = scan_notes(&notes, "vk1").unwrap_err();
    assert!(matches!(scan_error, zkscanner::NoteError::DuplicateNoteId));
}

#[test]
fn test_performance_integration() {
    // Test performance with larger datasets
    let mut notes = Vec::new();
    for i in 0..1000 {
        notes.push(Note {
            id: i,
            view_key: if i % 2 == 0 { "vk1".to_string() } else { "vk2".to_string() },
            data: format!("note{}", i),
        });
    }

    let start = std::time::Instant::now();
    let found = scan_notes(&notes, "vk1").unwrap();
    let duration = start.elapsed();

    assert_eq!(found.len(), 500);
    assert!(duration < std::time::Duration::from_secs(1));
} 