# CipherPay zkAudit

zkAudit is the compliance, proof-of-payment, and selective disclosure layer for CipherPay.
It allows DAOs, institutions, and users to verify payments and balances **without revealing unrelated private information**.

Built using RISC Zero zkVM for general-purpose zk execution.

## Features
- View key–based encrypted note scanning
- zkProof-of-Payment (zkPoP) generation
- Off-chain zk verification of past shielded transfers
- Web-based zk-audit verifier (optional UI)

## Architecture
```
cipherpay-zkaudit/
├── zkpop/                # RISC Zero zkVM proof logic
│   ├── zkpop.rs
│   └── Cargo.toml
├── scanner/              # View-key-based note scanner
│   └── decrypt_notes.rs
├── verifier/             # Optional web frontend (Next.js or React)
│   └── pages/
├── proofs/
│   └── tx_123.json       # Cached zkPoPs
├── README.md
```

## Usage
```bash
cd zkpop
cargo run --release
```

## Dependencies
- RISC Zero zkVM
- AES/NaCl crypto library
- Optional: Web verifier in React

## License
MIT © AppFounder Corp.
