# Reproducibility Guide

This file is the shortest path for reproducing the current paper-facing results.

## Environment

Set up the Python environment:

```bash
uv sync
```

Run the full local validation gate:

```bash
bash tools/quality.sh
```

If that passes, the repo is in a healthy starting state.

## Core Validation

Run the main numerical checks:

```bash
cargo test
.venv/bin/python tools/compare_scalar_reference.py
.venv/bin/python tools/compare_numpy_reference.py
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000
.venv/bin/python tools/wasm_full_field_metrics.py --width 64 --height 64 --steps 100
```

These support the correctness claims in the paper and
[experiment-log.md](experiment-log.md).

## Forward Performance Tables

Native Rust:

```bash
cargo run --release --bin bench_forward -- --grids 128,256,512 --steps 500 --trials 5
```

Scalar JavaScript:

```bash
node tools/bench_forward_js.mjs --grids 128,256,512 --steps 500 --trials 5
```

Scalar WASM:

```bash
bash tools/build_wasm_node.sh
node tools/bench_forward_wasm.mjs --grids 128,256,512 --steps 500 --trials 5
```

SIMD WASM:

```bash
bash tools/build_wasm_node_simd.sh
node tools/check_wasm_simd.mjs
node tools/bench_forward_wasm_simd.mjs --grids 128,256,512 --steps 500 --trials 5
```

Use these commands for the scalar/JS/WASM/SIMD performance numbers.

## Inverse And AD Tables

Grid-search baseline:

```bash
cargo run --release --bin inverse_grid -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --feed-min 0.058 --feed-max 0.063 --feed-count 11 \
  --kill-min 0.060 --kill-max 0.065 --kill-count 11
```

Gradient correctness:

```bash
cargo run --release --bin inverse_grad -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001

cargo run --release --bin inverse_ad -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Inverse overhead:

```bash
cargo run --release --bin bench_inverse -- \
  --grids 64,128,256 --steps 100 --trials 7 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Noise robustness:

```bash
cargo run --release --bin inverse_noise -- \
  --width 64 --height 64 --steps 100 \
  --noise-levels 0.000,0.001,0.005,0.010,0.020,0.050,0.100 \
  --seeds 24301,24589,51966,48879 \
  --feed-min 0.045 --feed-max 0.070 --feed-count 51 \
  --kill-min 0.055 --kill-max 0.070 --kill-count 31
```

AD optimizer vs grid search under noise:

```bash
cargo run --release --bin inverse_ad_opt -- \
  --width 64 --height 64 --steps 100 \
  --noise-levels 0.000,0.020,0.050,0.100 \
  --seeds 24301,24589,51966,48879 \
  --iterations 8 --learning-rate 0.0001 \
  --line-learning-rate 0.001 --line-shrink 0.5 \
  --line-armijo 0.0001 --line-min-step 0.00000001 \
  --line-max-backtracks 12 \
  --feed-min 0.045 --feed-max 0.070 --feed-count 51 \
  --kill-min 0.055 --kill-max 0.070 --kill-count 31
```

## Browser Render Results

Build browser WASM and start a local server:

```bash
bash tools/build_wasm_web.sh
python3 -m http.server 8000
```

Interactive render page:

```text
http://localhost:8000/www/render_bench.html
```

Headless render runner:

```bash
node tools/run_browser_render_bench.mjs --grid 512 --frames 300 --steps 250
```

For the exact manual/headless procedure and recorded medians, use
[manualcheck-browser-render.md](manualcheck-browser-render.md).

## Browser Inverse Results

Build browser WASM and start a local server:

```bash
bash tools/build_wasm_web.sh
python3 -m http.server 8000
```

Interactive inverse page:

```text
http://localhost:8000/www/inverse.html
```

Headless browser inverse runner:

```bash
node tools/run_browser_inverse_bench.mjs --grid 64 --steps 100 --iterations 8
```

The page runs the optimizer inside `www/inverse_worker.js`.

## Paper Files

Current paper source:

- [../paper/main.tex](../paper/main.tex)

Current compiled PDF:

- [../paper/grayscott_wasm_IEEE_Journal_Paper.pdf](../paper/grayscott_wasm_IEEE_Journal_Paper.pdf)

If a reproduced number changes, update:

1. [experiment-log.md](experiment-log.md)
2. the relevant paper table/text in [../paper/main.tex](../paper/main.tex)
3. any command summary in [../README.md](../README.md)
