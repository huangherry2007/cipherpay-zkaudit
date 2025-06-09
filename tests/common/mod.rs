use zkscanner::Note;
use zkpop::{generate_proof, verify_proof};

pub fn create_test_notes(count: usize, view_key: &str) -> Vec<Note> {
    (0..count)
        .map(|i| Note {
            id: i as u64,
            view_key: view_key.to_string(),
            data: format!("test_note_{}", i),
        })
        .collect()
}

pub fn generate_test_proofs(count: usize) -> Vec<(String, String)> {
    (0..count)
        .map(|i| {
            let input = format!("test_secret_{}", i);
            let proof = generate_proof(&input).unwrap();
            (input, serde_json::to_string(&proof).unwrap())
        })
        .collect()
}

pub fn verify_test_proofs(proofs: &[(String, String)]) -> bool {
    proofs.iter().all(|(input, proof_str)| {
        let proof: zkpop::Proof = serde_json::from_str(proof_str).unwrap();
        verify_proof(&proof, input).unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_utilities() {
        let notes = create_test_notes(5, "vk1");
        assert_eq!(notes.len(), 5);
        assert!(notes.iter().all(|n| n.view_key == "vk1"));

        let proofs = generate_test_proofs(3);
        assert_eq!(proofs.len(), 3);
        assert!(verify_test_proofs(&proofs));
    }
} 