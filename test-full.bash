#!/bin/bash
set -euxo pipefail

# Ensure stable builds.
cargo +stable check --all-targets --features __proxy
cargo +stable check --all-targets --features tracing,__proxy

# Ensure nightly builds.
cargo check --all-targets --features nightly,tracing,__proxy
cargo build --all-targets --features nightly,deny-unknown,__proxy

# Run nightly tests.
bash test.bash

# Run wasm tests.
bash test-wasm.bash
