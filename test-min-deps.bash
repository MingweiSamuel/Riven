#!/bin/bash
set -euxo pipefail

RUSTFLAGS='-Aunexpected_cfgs'
cargo update -Z direct-minimal-versions
cargo build --all-targets

./test.bash
