#!/bin/bash

echo "Building zkpop crate..."
cargo build --manifest-path ../zkpop/Cargo.toml

echo "Running integration tests..."
cargo test --manifest-path ../tests/integration_test.rs

echo "Deployment simulation complete. (Replace with real deployment steps)" 