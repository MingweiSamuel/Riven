#!/bin/bash
RGAPI_KEY="$(cat apikey.txt)" RUST_BACKTRACE=1 cargo test --features nightly -- --nocapture
