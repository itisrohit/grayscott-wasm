# grayscott-wasm

Research artifact for a Rust/WebAssembly Gray-Scott reaction-diffusion solver.

Current milestone: native scalar Rust solver plus a float32 NumPy reference.

## Commands

Set up Python tools:

```bash
uv sync
```

Install git hooks:

```bash
bash tools/install-hooks.sh
```

Run validation:

```bash
cargo test
.venv/bin/python reference/reference_scalar.py --width 64 --height 64 --steps 100
cargo run --example summary
.venv/bin/python tools/compare_scalar_reference.py
.venv/bin/python tools/compare_numpy_reference.py
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000
```

Run the full local quality gate:

```bash
bash tools/quality.sh
```

Run the native scalar forward benchmark:

```bash
cargo run --release --bin bench_forward -- --grids 128,256,512 --steps 500 --trials 5
```

Run the inverse-recovery grid-search baseline:

```bash
cargo run --release --bin inverse_grid -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --feed-min 0.058 --feed-max 0.063 --feed-count 11 \
  --kill-min 0.060 --kill-max 0.065 --kill-count 11
```

Run multi-regime inverse recovery:

```bash
cargo run --release --bin inverse_regimes -- \
  --width 64 --height 64 --steps 100 \
  --feed-min 0.045 --feed-max 0.070 --feed-count 51 \
  --kill-min 0.055 --kill-max 0.070 --kill-count 31
```

Run the inverse finite-difference gradient baseline:

```bash
cargo run --release --bin inverse_grad -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Run finite-difference gradient descent:

```bash
cargo run --release --bin inverse_opt -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --initial-feed 0.060 --initial-kill 0.063 \
  --learning-rate 0.0001 --epsilon 0.0001 --iterations 8
```

Compare forward-mode AD gradients against finite differences:

```bash
cargo run --release --bin inverse_ad -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Benchmark inverse-gradient overhead:

```bash
cargo run --release --bin bench_inverse -- \
  --grids 64,128,256 --steps 100 --trials 7 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Run Criterion statistical benchmarks for inverse-gradient overhead:

```bash
cargo bench --bench inverse_overhead
```

Run the JavaScript scalar forward benchmark:

```bash
node tools/bench_forward_js.mjs --grids 128,256,512 --steps 500 --trials 5
```

Build the Node.js WASM package:

```bash
bash tools/build_wasm_node.sh
```

Build the browser WASM package:

```bash
bash tools/build_wasm_web.sh
```

Check and benchmark the Node.js WASM package:

```bash
node tools/check_wasm_node.mjs
node tools/check_wasm_views.mjs
node tools/bench_forward_wasm.mjs --grids 128,256,512 --steps 500 --trials 5
.venv/bin/python tools/wasm_full_field_metrics.py --width 64 --height 64 --steps 100 500 1000
node tools/bench_wasm_boundary.mjs --grids 64,128,256 --steps 500 --trials 7
node tools/bench_wasm_views.mjs --grids 128,256,512 --trials 1000
node tools/bench_grayscale_render.mjs --grids 128,256,512 --trials 200
```

Run the browser render benchmark:

```bash
bash tools/build_wasm_web.sh
python3 -m http.server 8000
```

Then open:

```text
http://localhost:8000/www/render_bench.html
```

Check all JavaScript files:

```bash
bash tools/check_js.sh
```

Run the same gate through pre-commit:

```bash
PRE_COMMIT_HOME=.pre-commit-cache \
.venv/bin/pre-commit run --all-files
```

Run the NumPy reference directly:

```bash
.venv/bin/python reference/reference_numpy.py --width 64 --height 64 --steps 100
```
