#!/usr/bin/env bash
set -euo pipefail

rustup_bin="$(dirname "$(rustup which cargo)")"
export PATH="${rustup_bin}:${PATH}"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}"

wasm-pack build --target nodejs --release --out-dir pkg-node
