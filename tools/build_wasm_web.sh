#!/usr/bin/env bash
set -euo pipefail

rustup_bin="$(dirname "$(rustup which cargo)")"
export PATH="${rustup_bin}:${PATH}"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}"

wasm-pack build --target web --release --out-dir pkg-web
