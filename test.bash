#!/bin/bash
set -e

cargo +stable test --no-run
RGAPI_KEY="$(cat apikey.txt)" RUST_BACKTRACE=1 RUST_LOG=riven=trace cargo +nightly test --features nightly -- --nocapture
