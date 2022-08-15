#!/bin/bash
set -euxo pipefail

# Ensure stable builds.
cargo +stable check --all-targets
cargo +stable check --all-targets --features tracing

# Ensure nightly builds.
cargo check --all-targets --features nightly,tracing
cargo build --all-targets --features nightly,deny-unknown

# Run nightly tests.
bash test.bash
