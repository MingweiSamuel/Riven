#!/usr/bin/env bash
set -euxo pipefail

RIVEN_AUTOGEN_NONCE="$(date)" RIVEN_AUTOGEN_DEVMODE="src" cargo rustc -p riven
