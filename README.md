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
