#!/bin/bash
set -euxo pipefail

# Ensure stable builds.
cargo +stable check --all-targets --features riven/__proxy
cargo +stable check --all-targets --features riven/metrics,riven/tracing,riven/__proxy

# Ensure nightly builds.
cargo check --all-targets --features riven/nightly,riven/metrics,riven/tracing,riven/__proxy
cargo build --all-targets --features riven/nightly,riven/deny-unknown,riven/eserde,riven/__proxy

# Run nightly tests.
bash test.bash

# Run wasm tests.
bash test-wasm.bash
