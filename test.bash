#!/bin/bash
set -euxo pipefail

RGAPI_KEY="$(cat apikey.txt)" cargo test --no-fail-fast \
    --features riven/nightly,riven/tracing,riven/eserde,riven/deny-unknown -- --nocapture
