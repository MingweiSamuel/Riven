#!/bin/bash
set -euxo pipefail

export RGAPI_KEY="$(cat apikey.txt)"

cd riven

# Ensure builds with tracing, metrics.
wasm-pack build -- --features riven/nightly,riven/tracing,riven/metrics

# Run tests.
wasm-pack test --node -- --features riven/nightly,riven/deny-unknown,riven/eserde
