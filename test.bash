#!/bin/bash
RGAPI_KEY="$(cat apikey.txt)" RUST_BACKTRACE=1 RUST_LOG=riven=trace cargo +nightly test --features nightly,deny-unknown-fields $1 -- --nocapture
