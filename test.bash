#!/bin/bash
set -euxo pipefail

RGAPI_KEY="$(cat apikey.txt)" RUST_BACKTRACE=1 RUST_LOG=riven=debug cargo test --no-fail-fast --features nightly,deny-unknown -- --nocapture
