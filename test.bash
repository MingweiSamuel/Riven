#!/bin/bash
set -e

cargo +stable test -- --nocapture
RGAPI_KEY="$(cat apikey.txt)" RUST_BACKTRACE=1 RUST_LOG=riven=debug cargo +nightly test --features nightly -- --nocapture
