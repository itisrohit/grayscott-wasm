# grayscott-wasm

Research artifact for a Rust/WebAssembly Gray-Scott reaction-diffusion solver.

Current milestone: native scalar Rust solver plus a float32 NumPy reference.

## Commands

Set up Python tools:

```bash
uv sync
```

Run validation:

```bash
cargo test
.venv/bin/python reference/reference_scalar.py --width 64 --height 64 --steps 100
cargo run --example summary
.venv/bin/python tools/compare_scalar_reference.py
.venv/bin/python tools/compare_numpy_reference.py
```

Run the NumPy reference directly:

```bash
.venv/bin/python reference/reference_numpy.py --width 64 --height 64 --steps 100
```
