#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

.venv/bin/ruff format --check .
.venv/bin/ruff check .
bash tools/check_js.sh
bash tools/build_wasm_node.sh
node tools/check_wasm_node.mjs

.venv/bin/python tools/compare_scalar_reference.py
.venv/bin/python tools/compare_numpy_reference.py
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000
