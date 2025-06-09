#!/bin/bash
set -e

cd zkpop/host

echo "Running Merkle proof tests..."
cargo run --release -- merkle

echo "Running Audit proof tests..."
cargo run --release -- audit

echo "All tests completed." 