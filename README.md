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
