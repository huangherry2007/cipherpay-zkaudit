use crate::utils::{Hash, hex_to_bytes32};
pub const DEPTH: usize = 32;

pub struct MerkleTestVector {
    pub root: Hash,
    pub leaf: Hash,
    pub path_elements: [Hash; DEPTH],
    pub path_indices: [u8; DEPTH],
}

pub struct AuditTestVector {
    pub note_commitment: Hash,
    pub view_key: Hash,
    pub amount: u64,
    pub timestamp: u64,
    pub purpose: u64,
    pub audit_id: Hash,
    pub merkle_root: Hash,
}

pub struct TransferTestVector {
    pub amount: u64,
    pub sender: Hash,
    pub receiver: Hash,
}

pub struct WithdrawTestVector {
    pub amount: u64,
    pub recipient: Hash,
}

pub struct NullifierTestVector {
    pub nullifier: Hash,
    pub secret: Hash,
}

pub struct ZkConditionTestVector {
    pub condition_type: u8,
    pub value: u64,
}

pub struct ZkSplitTestVector {
    pub recipients: [Hash; 2],
    pub amounts: [u64; 2],
}

pub struct ZkStreamTestVector {
    pub stream_id: Hash,
    pub total_amount: u64,
}

pub fn merkle_test_vectors() -> Vec<MerkleTestVector> {
    vec![
        MerkleTestVector {
            root: hex_to_bytes32("0x1234567890abcdef"),
            leaf: hex_to_bytes32("0xfedcba0987654321"),
            path_elements: [[0u8; 32]; DEPTH],
            path_indices: [0u8; DEPTH],
        },
        MerkleTestVector {
            root: hex_to_bytes32("0xabcdefabcdefabcd"),
            leaf: hex_to_bytes32("0x1111111111111111"),
            path_elements: [[1u8; 32]; DEPTH],
            path_indices: [1u8; DEPTH],
        },
    ]
}

pub fn audit_test_vectors() -> Vec<AuditTestVector> {
    vec![
        AuditTestVector {
            note_commitment: hex_to_bytes32("0x11111111111111111111111111111111"),
            view_key: hex_to_bytes32("0x22222222222222222222222222222222"),
            amount: 100,
            timestamp: 1234560000,
            purpose: 1,
            audit_id: [0u8; 32], // Placeholder, should match guest computation
            merkle_root: hex_to_bytes32("0x33333333333333333333333333333333"),
        },
        AuditTestVector {
            note_commitment: hex_to_bytes32("0xabcdefabcdefabcdefabcdefabcdefab"),
            view_key: hex_to_bytes32("0x44444444444444444444444444444444"),
            amount: 200,
            timestamp: 1234550000,
            purpose: 2,
            audit_id: [0u8; 32], // Placeholder, should match guest computation
            merkle_root: hex_to_bytes32("0x55555555555555555555555555555555"),
        },
    ]
}

pub fn transfer_test_vectors() -> Vec<TransferTestVector> {
    vec![
        // Valid: amount > 0, commitments/keys are nonzero
        TransferTestVector {
            amount: 100,
            sender: hex_to_bytes32("0xaaaa"),
            receiver: hex_to_bytes32("0xbbbb"),
        },
        // Invalid: amount == 0
        TransferTestVector {
            amount: 0,
            sender: hex_to_bytes32("0xaaaa"),
            receiver: hex_to_bytes32("0xbbbb"),
        },
        // Invalid: sender and receiver are the same
        TransferTestVector {
            amount: 50,
            sender: hex_to_bytes32("0xcccc"),
            receiver: hex_to_bytes32("0xcccc"),
        },
    ]
}

pub fn withdraw_test_vectors() -> Vec<WithdrawTestVector> {
    vec![
        // Valid: amount > 0, withdrawal_amount matches, recipient nonzero
        WithdrawTestVector {
            amount: 50,
            recipient: hex_to_bytes32("0xcccc"),
        },
        // Invalid: amount == 0
        WithdrawTestVector {
            amount: 0,
            recipient: hex_to_bytes32("0xcccc"),
        },
        // Invalid: recipient is zero
        WithdrawTestVector {
            amount: 50,
            recipient: [0u8; 32],
        },
    ]
}

pub fn nullifier_test_vectors() -> Vec<NullifierTestVector> {
    vec![
        // Valid: nonzero values
        NullifierTestVector {
            nullifier: hex_to_bytes32("0xdddd"),
            secret: hex_to_bytes32("0xeeee"),
        },
        // Invalid: nullifier is zero
        NullifierTestVector {
            nullifier: [0u8; 32],
            secret: hex_to_bytes32("0xeeee"),
        },
        // Invalid: secret is zero
        NullifierTestVector {
            nullifier: hex_to_bytes32("0xdddd"),
            secret: [0u8; 32],
        },
    ]
}

pub fn zkcondition_test_vectors() -> Vec<ZkConditionTestVector> {
    vec![
        // Valid: type 0, value > 0
        ZkConditionTestVector { condition_type: 0, value: 12345 },
        // Valid: type 1, value > 0
        ZkConditionTestVector { condition_type: 1, value: 1 },
        // Invalid: type 2 (not allowed)
        ZkConditionTestVector { condition_type: 2, value: 100 },
        // Invalid: value == 0
        ZkConditionTestVector { condition_type: 0, value: 0 },
    ]
}

pub fn zksplit_test_vectors() -> Vec<ZkSplitTestVector> {
    vec![
        // Valid: unique, nonzero, sum > 0
        ZkSplitTestVector {
            recipients: [hex_to_bytes32("0x1111"), hex_to_bytes32("0x2222")],
            amounts: [60, 40],
        },
        // Invalid: duplicate recipients
        ZkSplitTestVector {
            recipients: [hex_to_bytes32("0x1111"), hex_to_bytes32("0x1111")],
            amounts: [60, 40],
        },
        // Invalid: zero recipient
        ZkSplitTestVector {
            recipients: [hex_to_bytes32("0x1111"), [0u8; 32]],
            amounts: [60, 40],
        },
        // Invalid: sum == 0
        ZkSplitTestVector {
            recipients: [hex_to_bytes32("0x1111"), hex_to_bytes32("0x2222")],
            amounts: [0, 0],
        },
    ]
}

pub fn zkstream_test_vectors() -> Vec<ZkStreamTestVector> {
    vec![
        // Valid: positive amount, valid id
        ZkStreamTestVector {
            stream_id: hex_to_bytes32("0x3333"),
            total_amount: 1000,
        },
        // Invalid: zero amount
        ZkStreamTestVector {
            stream_id: hex_to_bytes32("0x3333"),
            total_amount: 0,
        },
        // Invalid: zero id
        ZkStreamTestVector {
            stream_id: [0u8; 32],
            total_amount: 1000,
        },
    ]
} 