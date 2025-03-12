#!/bin/bash
set -euxo pipefail

RGAPI_KEY="$(cat apikey.txt)" RUST_BACKTRACE=full RUST_LOG=riven=debug cargo test --no-fail-fast --features nightly,tracing,eserde,deny-unknown,eserde -- --nocapture
