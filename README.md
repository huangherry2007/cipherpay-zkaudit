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
│   ├── guest/            # Guest program (runs in zkVM)
│   │   └── main.rs
│   ├── host/             # Host program (proves/verifies)
│   │   └── main.rs
│   └── Cargo.toml
├── zkscanner/            # View-key-based note scanner
│   └── decrypt_notes.rs
├── zkverifier/           # Optional web frontend (Next.js or React)
│   └── pages/
├── zkproofs/             # Cached zkPoPs
├── scripts/              # Utility scripts
├── tests/                # Integration and end-to-end tests
├── docs/                 # Documentation, whitepapers
├── config/               # Configuration files
├── README.md
```

## Project Structure

- zkpop/guest/: RISC Zero guest program (no_std, runs in zkVM)
- zkpop/host/: RISC Zero host program (proves/verifies)
- zkscanner/: note scanning logic
- zkverifier/: web frontend for zk-audit
- zkproofs/: cached zkPoPs (proofs of payment)
- scripts/: utility scripts (deployment, setup, etc.)
- tests/: integration and end-to-end tests
- docs/: documentation, whitepapers, and architecture diagrams
- config/: configuration files for different environments

## Usage
```bash
cd zkpop/host
cargo run --release
```

## Dependencies
- RISC Zero zkVM
- AES/NaCl crypto library
- Optional: Web verifier in React

## Setup

### Prerequisites
- Rust (latest stable)
- Node.js 16+ (for web verifier)
- RISC Zero toolchain

### Installation
1. Clone the repository:
```bash
git clone https://github.com/your-org/cipherpay-zkaudit.git
cd cipherpay-zkaudit
```

2. Install Rust dependencies:
```bash
cd zkpop
cargo build
```

3. (Optional) Install web verifier dependencies:
```bash
cd zkverifier
npm install
```

## Development

### Building
```bash
# Build RISC Zero guest program
cd zkpop/guest
cargo build

# Build host program
cd ../host
cargo build
```

### Testing
```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*'
```

### Running the Web Verifier
```bash
cd zkverifier
npm run dev
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines
- Follow Rust best practices and style guide
- Write tests for new features
- Update documentation for API changes
- Keep the ZKP logic in sync between RISC Zero and Circom implementations

## Security

- Report security vulnerabilities to security@your-org.com
- Do not disclose security-related issues publicly

## License
MIT © AppFounder Corp.

## Contact

- Project Link: [https://github.com/your-org/cipherpay-zkaudit](https://github.com/your-org/cipherpay-zkaudit)
- Documentation: [https://docs.your-org.com/cipherpay-zkaudit](https://docs.your-org.com/cipherpay-zkaudit)
