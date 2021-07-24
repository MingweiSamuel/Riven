#!/bin/bash
set -e

# Ensure stable builds.
cargo +stable test --no-run
cargo +stable test --no-run --features tracing

# Ensure nightly builds.
cargo +nightly test --no-run --features nightly,tracing

# Run tests on nightly.
RGAPI_KEY="$(cat apikey.txt)" RUST_BACKTRACE=1 RUST_LOG=riven=trace cargo +nightly test --features nightly -- --nocapture
