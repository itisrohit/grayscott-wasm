#!/usr/bin/env bash
set -euo pipefail

rustup_bin="$(dirname "$(rustup which cargo)")"
export PATH="${rustup_bin}:${PATH}"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}"
export RUSTFLAGS="${RUSTFLAGS:-} -C target-feature=+simd128"

wasm-pack build --target nodejs --release --out-dir pkg-node-simd
