#!/bin/bash
set -e

# Build Merkle guest
cd zkpop/guest
cargo build --release --bin main_merkle

# Build Audit guest
cargo build --release --bin main_audit

# Build host
cd ../host
cargo build --release

echo "All binaries built successfully." 