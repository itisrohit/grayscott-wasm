# Experiment Log

This file records measured results and validation gates as the research artifact
evolves. Keep it factual: commands, environment, metrics, tolerances, and observed
numbers.

Current date: 2026-04-26

---

## Environment

Host:

- OS: macOS, local development machine
- Rust: `rustc 1.94.0 (4a4ef493e 2026-03-02) (Homebrew)`
- Cargo: `cargo 1.94.0 (Homebrew)`
- Python: `Python 3.13.5`
- uv: `uv 0.8.9 (68c0bf8a2 2025-08-11)`
- NumPy: `2.4.4`, installed in project `.venv`

Python dependency setup:

```bash
uv --cache-dir .uv-cache sync
```

Note: `uv run` currently panics in the sandboxed environment on macOS system
configuration access. The project `.venv` created by `uv` works, so validation
commands use `.venv/bin/python`.

---

## Solver Baseline

Implementation:

- Native Rust scalar solver
- Structure-of-arrays memory layout:
  - `u: Vec<f32>`
  - `v: Vec<f32>`
  - `next_u: Vec<f32>`
  - `next_v: Vec<f32>`
- 5-point Laplacian
- Periodic boundary conditions
- Explicit Euler integration
- Default parameters:
  - `F = 0.060`
  - `k = 0.062`
  - `Du = 0.16`
  - `Dv = 0.08`
  - `dt = 1.0`

Initial condition for current regression:

- Grid: `64 x 64`
- `u = 1.0` everywhere
- `v = 0.0` everywhere
- Center square seed at `(32, 32)` with radius `5`
- Seed values:
  - `u = 0.50`
  - `v = 0.25`
- Steps: `100`

---

## Metrics

Current scalar validation metrics:

- `u_min`
- `u_max`
- `u_mean`
- `v_min`
- `v_max`
- `v_mean`

Current tolerances:

- Rust regression test tolerance: `1e-6`
- Rust vs NumPy float32 summary tolerance: `1e-6`
- Rust vs dependency-free Python scalar summary tolerance: `1e-5`

Why two Python tolerances:

- NumPy reference forces `float32` arrays and is expected to match Rust more
  closely.
- Dependency-free Python scalar reference uses Python `float` values, effectively
  `f64`, so a looser tolerance is appropriate.

Future correctness metrics to add:

- MAE over full `u` field: added for `64 x 64`
- MAE over full `v` field: added for `64 x 64`
- RMSE over full fields: added for `64 x 64`
- Max absolute error: added for `64 x 64`
- Pattern similarity metric, optional

---

## Baseline Results

Command:

```bash
cargo run --example summary
```

Observed Rust scalar output:

```text
u_min=0.306591392 u_max=1.000000000 u_mean=0.980694592
v_min=0.000000000 v_max=0.420273542 v_mean=0.009767476
```

Command:

```bash
.venv/bin/python reference/reference_numpy.py --width 64 --height 64 --steps 100
```

Observed NumPy float32 reference output:

```text
u_min=0.306591392 u_max=1.000000000 u_mean=0.980694592
v_min=0.000000000 v_max=0.420273542 v_mean=0.009767476
```

Command:

```bash
.venv/bin/python reference/reference_scalar.py --width 64 --height 64 --steps 100
```

Observed dependency-free Python scalar reference output:

```json
{
  "u": {
    "max": 0.9999999999999903,
    "mean": 0.980694556753063,
    "min": 0.3065914507299372
  },
  "v": {
    "max": 0.4202734602669786,
    "mean": 0.009767467013787131,
    "min": 5.592191259653523e-26
  }
}
```

---

## Validation Commands

Rust unit and regression tests:

```bash
cargo test
```

Observed result:

```text
4 unit tests passed
1 regression test passed
0 failed
```

Rust vs NumPy float32 reference:

```bash
.venv/bin/python tools/compare_numpy_reference.py
```

Observed result:

```text
Rust scalar summary matches NumPy float32 reference within 1e-6.
```

Rust vs dependency-free Python scalar reference:

```bash
.venv/bin/python tools/compare_scalar_reference.py
```

Observed result:

```text
Rust scalar summary matches Python scalar reference within 1e-5.
```

---

## Current Validation Status

Passed:

- Uniform steady state remains unchanged.
- Hand-checked one-step periodic update matches expected center-cell values.
- Standard seeded `64 x 64`, 100-step Rust regression matches stored summary
  values.
- Rust scalar summary matches NumPy `float32` reference within `1e-6`.
- Rust scalar summary matches dependency-free Python scalar reference within
  `1e-5`.
- Full-field Rust scalar output matches NumPy `float32` reference closely for
  `64 x 64` at `100`, `500`, and `1000` steps.
- Full-field Rust scalar output matches NumPy `float32` reference closely for
  `128 x 128`, `256 x 256`, and `512 x 512` at `100`, `500`, and `1000`
  steps.

Not yet done:

- Native Rust performance benchmark.
- JavaScript baseline.
- WASM build.
- SIMD build.
- Automatic differentiation.
- Gradient checks.
- Inverse recovery.

---

## Browser Render Benchmark Harness

Implementation:

- Browser WASM build:
  - `tools/build_wasm_web.sh`
  - output directory: `pkg-web/`
- Browser benchmark page:
  - `www/render_bench.html`
  - `www/render_bench.js`
  - `www/render_bench.css`

Commands:

```bash
bash tools/build_wasm_web.sh
python3 -m http.server 8000
```

Open:

```text
http://localhost:8000/www/render_bench.html
```

Metrics recorded by the page:

- `Float32Array` field view to reusable `Uint8ClampedArray` RGBA buffer.
- `new ImageData(pixels, width, height)`.
- 2D canvas `putImageData`.
- OffscreenCanvas `putImageData`, when supported.
- OffscreenCanvas `transferToImageBitmap` plus `bitmaprenderer` transfer, when
  supported.

Current result:

- Harness added.
- Manual Chrome measurements recorded for `128 x 128`, `256 x 256`, and
  `512 x 512`.

Manual browser environment:

- Browser: Chrome `146.0.0.0`
- OS reported by user agent: `Macintosh; Intel Mac OS X 10_15_7`
- OffscreenCanvas support: `true`
- `bitmaprenderer` support: `true`
- User agent:

```text
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36
```

Manual benchmark settings:

- Frames: `300`
- Warmup steps: `250`
- Runs per grid: `3`
- Reported value: median ms/frame

Observed median output:

| Grid | Field to RGBA | new ImageData | 2D putImageData | OffscreenCanvas putImageData | OffscreenCanvas ImageBitmap transfer | Checksum |
|---|---:|---:|---:|---:|---:|---:|
| 128x128 | 0.084000 | 0.000333 | 0.010667 | 0.009333 | 0.016000 | 92126 |
| 256x256 | 0.205333 | 0.000000 | 0.027333 | 0.021000 | 0.027000 | 369296 |
| 512x512 | 0.817000 | 0.000333 | 0.093333 | 0.094333 | 0.099667 | 1475932 |

Raw runs:

| Grid | Run | Field to RGBA | new ImageData | 2D putImageData | OffscreenCanvas putImageData | OffscreenCanvas ImageBitmap transfer |
|---|---:|---:|---:|---:|---:|---:|
| 128x128 | 1 | 0.080000 | 0.000667 | 0.010000 | 0.008667 | 0.016333 |
| 128x128 | 2 | 0.084000 | 0.000333 | 0.010667 | 0.009333 | 0.016000 |
| 128x128 | 3 | 0.084333 | 0.000333 | 0.011000 | 0.009333 | 0.016000 |
| 256x256 | 1 | 0.205333 | 0.000000 | 0.028333 | 0.021000 | 0.027000 |
| 256x256 | 2 | 0.205000 | 0.000667 | 0.025000 | 0.021000 | 0.026667 |
| 256x256 | 3 | 0.207667 | 0.000000 | 0.027333 | 0.021333 | 0.027000 |
| 512x512 | 1 | 0.817000 | 0.000333 | 0.090333 | 0.089667 | 0.124667 |
| 512x512 | 2 | 0.817000 | 0.000000 | 0.093333 | 0.095000 | 0.099667 |
| 512x512 | 3 | 0.819000 | 0.000667 | 0.096000 | 0.094333 | 0.099667 |

Interpretation:

- Field-to-RGBA conversion dominates the measured rendering-side cost at
  `512 x 512`, at `0.817000 ms/frame`.
- `ImageData` construction is negligible in this Chrome run, but the benchmark
  resolution is coarse enough that values near zero should not be overinterpreted.
- `putImageData` remains below `0.1 ms/frame` at `512 x 512` for both main-canvas
  and OffscreenCanvas paths in this environment.
- OffscreenCanvas/ImageBitmap transfer is supported, but it is not faster than
  direct `putImageData` in the `512 x 512` median result.
- These are single-browser, single-machine manual results. Browser-rendering
  claims should remain qualified until repeated on at least one more browser or
  machine.

---

## Next Validation Upgrade

The first full-field correctness milestone now compares full fields, not only
summary stats.

Command:

```bash
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000
```

Observed output:

| Grid | Steps | Regime | u_MAE | v_MAE | u_RMSE | v_RMSE | u_MaxErr | v_MaxErr |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| 64x64 | 100 | F=0.060, k=0.062 | 2.184e-08 | 2.080e-09 | 4.181e-08 | 1.401e-08 | 2.384e-07 | 1.937e-07 |
| 64x64 | 500 | F=0.060, k=0.062 | 3.744e-08 | 1.014e-08 | 6.996e-08 | 3.951e-08 | 5.364e-07 | 3.725e-07 |
| 64x64 | 1000 | F=0.060, k=0.062 | 4.238e-08 | 1.755e-08 | 8.347e-08 | 5.602e-08 | 5.960e-07 | 5.364e-07 |

Implementation note:

- Rust final fields are exported as little-endian raw `f32` arrays by
  `examples/export_fields.rs`.
- NumPy loads those arrays as `dtype="<f4"` and compares against the reference
  solver directly.
- Initial conditions and parameters are identical.

Next upgrade:

- Add at least three more parameter regimes.

---

## Multi-Grid Full-Field Validation

Commands:

```bash
.venv/bin/python tools/full_field_metrics.py --width 128 --height 128 --steps 100 500 1000
.venv/bin/python tools/full_field_metrics.py --width 256 --height 256 --steps 100 500 1000
.venv/bin/python tools/full_field_metrics.py --width 512 --height 512 --steps 100 500 1000
```

Observed output:

| Grid | Steps | Regime | u_MAE | v_MAE | u_RMSE | v_RMSE | u_MaxErr | v_MaxErr |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| 128x128 | 100 | F=0.060, k=0.062 | 5.461e-09 | 5.200e-10 | 2.091e-08 | 7.005e-09 | 2.384e-07 | 1.937e-07 |
| 128x128 | 500 | F=0.060, k=0.062 | 9.361e-09 | 2.535e-09 | 3.498e-08 | 1.975e-08 | 5.364e-07 | 3.725e-07 |
| 128x128 | 1000 | F=0.060, k=0.062 | 1.260e-08 | 4.387e-09 | 4.402e-08 | 2.801e-08 | 5.960e-07 | 5.364e-07 |
| 256x256 | 100 | F=0.060, k=0.062 | 1.365e-09 | 1.300e-10 | 1.045e-08 | 3.502e-09 | 2.384e-07 | 1.937e-07 |
| 256x256 | 500 | F=0.060, k=0.062 | 2.340e-09 | 6.337e-10 | 1.749e-08 | 9.877e-09 | 5.364e-07 | 3.725e-07 |
| 256x256 | 1000 | F=0.060, k=0.062 | 3.150e-09 | 1.097e-09 | 2.201e-08 | 1.400e-08 | 5.960e-07 | 5.364e-07 |
| 512x512 | 100 | F=0.060, k=0.062 | 3.413e-10 | 3.250e-11 | 5.226e-09 | 1.751e-09 | 2.384e-07 | 1.937e-07 |
| 512x512 | 500 | F=0.060, k=0.062 | 5.850e-10 | 1.584e-10 | 8.745e-09 | 4.938e-09 | 5.364e-07 | 3.725e-07 |
| 512x512 | 1000 | F=0.060, k=0.062 | 7.876e-10 | 2.742e-10 | 1.101e-08 | 7.002e-09 | 5.960e-07 | 5.364e-07 |

Interpretation:

- Full-field agreement remains tight through `512 x 512` and `1000` steps.
- Max error remains below `6e-7` for both fields in this regime.
- The decreasing MAE at larger grids is expected here because the same fixed-size
  seed occupies a smaller fraction of the domain.

Next validation upgrade:

- Add native Rust performance benchmark.

---

## Native Forward Benchmark Method

Command:

```bash
cargo run --release --bin bench_forward -- --grids 128,256,512 --steps 500 --trials 5
```

Benchmark method:

- Native Rust scalar solver only.
- Release build.
- Grids: `128 x 128`, `256 x 256`, `512 x 512`.
- Steps per trial: `500`.
- Trials per grid: `5`.
- Warmup: `25` steps before timed trials.
- Initial condition: same centered square seed used in correctness tests.
- Metrics:
  - median milliseconds per step,
  - min milliseconds per step,
  - max milliseconds per step,
  - median steps per second,
  - cell updates per second,
  - checksum of final `u` and `v` fields.

The checksum is reported to make sure the benchmark performs the simulation work
and to catch accidental behavioral changes.

Observed output:

| Grid | Steps | Trials | Median ms/step | Min ms/step | Max ms/step | Median steps/s | Cells/s | Checksum |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 128x128 | 500 | 5 | 0.050298 | 0.044711 | 0.070395 | 19881.67 | 3.257e8 | 16282.767486 |
| 256x256 | 500 | 5 | 0.176637 | 0.176153 | 0.176960 | 5661.33 | 3.710e8 | 65434.767486 |
| 512x512 | 500 | 5 | 0.707058 | 0.703682 | 0.710550 | 1414.31 | 3.708e8 | 262042.767486 |

Interpretation:

- Throughput is roughly flat around `3.7e8` cell updates/s for `256 x 256` and
  `512 x 512`.
- The `128 x 128` case has more timing noise because each step is very short.
- This is only the native scalar Rust baseline. It is not yet a JavaScript, WASM,
  or SIMD comparison.

---

## JavaScript Forward Benchmark Method

Command:

```bash
node tools/bench_forward_js.mjs --grids 128,256,512 --steps 500 --trials 5
```

Benchmark method:

- Node.js scalar JavaScript solver.
- `Float32Array` storage for `u`, `v`, `next_u`, and `next_v`.
- Same 5-point Laplacian, periodic boundaries, explicit Euler update, parameters,
  seed, grids, steps, trials, and warmup as the native Rust scalar benchmark.

Observed output:

| Grid | Steps | Trials | Median ms/step | Min ms/step | Max ms/step | Median steps/s | Cells/s | Checksum |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 128x128 | 500 | 5 | 0.083996 | 0.082724 | 0.085903 | 11905.40 | 1.951e8 | 16282.767663 |
| 256x256 | 500 | 5 | 0.331804 | 0.322597 | 0.340275 | 3013.83 | 1.975e8 | 65434.767663 |
| 512x512 | 500 | 5 | 1.289701 | 1.274529 | 2.264158 | 775.37 | 2.033e8 | 262042.767663 |

Initial Rust-vs-JS scalar comparison:

| Grid | Rust median ms/step | JS median ms/step | Rust speedup |
|---|---:|---:|---:|
| 128x128 | 0.050298 | 0.083996 | 1.67x |
| 256x256 | 0.176637 | 0.331804 | 1.88x |
| 512x512 | 0.707058 | 1.289701 | 1.82x |

Interpretation:

- Native Rust scalar is faster than Node.js scalar in this baseline, but the
  speedup is about `1.7x-1.9x`, not the `5x-15x` sometimes claimed for WASM vs
  naive JavaScript.
- This is a CPU-native Rust comparison, not WASM yet. The browser/WASM comparison
  must be measured separately.
- JS checksum differs slightly from Rust because JavaScript arithmetic is double
  precision with `Float32Array` stores, while Rust computes directly with `f32`.

---

## Node.js WASM Forward Benchmark Method

Build command:

```bash
bash tools/build_wasm_node.sh
```

Correctness smoke check:

```bash
node tools/check_wasm_node.mjs
```

Observed output:

```text
WASM checksum ok: 4056.932528740907
```

Benchmark command:

```bash
node tools/bench_forward_wasm.mjs --grids 128,256,512 --steps 500 --trials 5
```

Benchmark method:

- Rust scalar solver compiled to WebAssembly with `wasm-pack`.
- `wasm-pack build --target nodejs --release --out-dir pkg-node`.
- Node.js loads the generated CommonJS package via `createRequire`.
- Same seed, parameters, grids, steps, warmup, and trials as native Rust and JS
  scalar benchmarks.
- Timed loop calls `WasmGrayScott.run(steps)` once per trial, so per-step JS/WASM
  boundary overhead is not included.

Observed output:

| Grid | Steps | Trials | Median ms/step | Min ms/step | Max ms/step | Median steps/s | Cells/s | Checksum |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 128x128 | 500 | 5 | 0.066056 | 0.065813 | 0.066204 | 15138.57 | 2.480e8 | 16282.767486 |
| 256x256 | 500 | 5 | 0.263475 | 0.262124 | 0.264311 | 3795.43 | 2.487e8 | 65434.767486 |
| 512x512 | 500 | 5 | 1.053084 | 1.046151 | 1.060119 | 949.59 | 2.489e8 | 262042.767486 |

Initial native Rust vs WASM vs JS scalar comparison:

| Grid | Native Rust ms/step | WASM ms/step | JS ms/step | WASM vs JS | Native Rust vs WASM |
|---|---:|---:|---:|---:|---:|
| 128x128 | 0.050298 | 0.066056 | 0.083996 | 1.27x faster | 1.31x faster |
| 256x256 | 0.176637 | 0.263475 | 0.331804 | 1.26x faster | 1.49x faster |
| 512x512 | 0.707058 | 1.053084 | 1.289701 | 1.22x faster | 1.49x faster |

Interpretation:

- Scalar WASM is faster than scalar JavaScript in Node.js, but only by about
  `1.22x-1.27x` in this setup.
- Native Rust remains faster than scalar WASM by about `1.4x-1.6x`.
- These results do not support an inflated `5x-15x` scalar WASM speedup claim.
- The benchmark avoids per-step boundary overhead by calling `run(steps)` once per
  trial. A separate benchmark should measure per-step JS/WASM boundary overhead if
  the browser UI calls `step()` repeatedly.

---

## WASM Full-Field Validation

Command:

```bash
.venv/bin/python tools/wasm_full_field_metrics.py --width 64 --height 64 --steps 100 500 1000
```

Method:

- Build the Node.js WASM package with `tools/build_wasm_node.sh`.
- Export final `u` and `v` fields from `WasmGrayScott` as little-endian raw
  `f32` arrays.
- Load those arrays in Python and compare against the NumPy `float32` reference.

Observed output:

| Grid | Steps | u_MAE | v_MAE | u_RMSE | v_RMSE | u_MaxErr | v_MaxErr |
|---|---:|---:|---:|---:|---:|---:|---:|
| 64x64 | 100 | 2.184e-08 | 2.080e-09 | 4.181e-08 | 1.401e-08 | 2.384e-07 | 1.937e-07 |
| 64x64 | 500 | 3.744e-08 | 1.014e-08 | 6.996e-08 | 3.951e-08 | 5.364e-07 | 3.725e-07 |
| 64x64 | 1000 | 4.238e-08 | 1.755e-08 | 8.347e-08 | 5.602e-08 | 5.960e-07 | 5.364e-07 |

Interpretation:

- WASM full-field output agrees with the NumPy `float32` reference at the same
  error level as native Rust for this case.
- This validates that the wasm-bindgen wrapper is not changing numerical results.

---

## WASM Boundary Overhead Benchmark

Command:

```bash
node tools/bench_wasm_boundary.mjs --grids 64,128,256 --steps 500 --trials 7
```

Method:

- Bulk mode calls `WasmGrayScott.run(steps)` once per trial.
- Per-step-call mode calls `WasmGrayScott.step()` from JS `steps` times.
- Both modes use the same final checksum.

Observed output:

| Grid | Steps | Trials | Bulk ms/step | Per-step-call ms/step | Boundary overhead | Bulk checksum | Step checksum |
|---|---:|---:|---:|---:|---:|---:|---:|
| 64x64 | 500 | 7 | 0.017932 | 0.017917 | 1.00x | 3994.767486 | 3994.767486 |
| 128x128 | 500 | 7 | 0.068324 | 0.066201 | 0.97x | 16282.767486 | 16282.767486 |
| 256x256 | 500 | 7 | 0.262825 | 0.262816 | 1.00x | 65434.767486 | 65434.767486 |

Interpretation:

- For `64x64` and larger grids, JS/WASM boundary overhead is not visible above
  timing noise because each step performs enough stencil work.
- This does not prove boundary overhead is zero. A separate microbenchmark with a
  no-op exported function would be needed to measure pure call overhead.
- For the current solver, chunking steps for UI responsiveness can be chosen based
  on rendering needs rather than measured call overhead at these grid sizes.

---

## WASM Zero-Copy Field View Benchmark

Correctness command:

```bash
node tools/check_wasm_views.mjs
```

Observed output:

```text
WASM zero-copy views match copied fields exactly.
```

Benchmark command:

```bash
node tools/bench_wasm_views.mjs --grids 128,256,512 --trials 1000
```

Method:

- Copy path calls `u_values()` and `v_values()`, which copy fields out of WASM
  memory.
- View path calls `u_view()` and `v_view()`, which return `Float32Array` views over
  WASM memory without copying field data.
- The benchmark samples three values from each field so both paths consume the
  returned data.

Observed output:

| Grid | Cells | Trials | Copy ms/trial | View ms/trial | View speedup | Copy checksum | View checksum |
|---|---:|---:|---:|---:|---:|---:|---:|
| 128x128 | 16384 | 1000 | 0.028776 | 0.001195 | 24.08x | 3000.000000 | 3000.000000 |
| 256x256 | 65536 | 1000 | 0.033051 | 0.000376 | 87.96x | 3000.000000 | 3000.000000 |
| 512x512 | 262144 | 1000 | 0.139924 | 0.000249 | 560.82x | 3000.000000 | 3000.000000 |

Interpretation:

- Zero-copy field views remove the full-field copy cost and are the correct path
  for browser rendering.
- The view benchmark measures view creation plus sampling, not full image
  rendering.
- `Float32Array` views into WASM memory are invalidated if the WASM memory grows.
  Rendering code should recreate views after operations that can allocate or grow
  memory.

---

## Grayscale Render Buffer Benchmark

Command:

```bash
node tools/bench_grayscale_render.mjs --grids 128,256,512 --trials 1000
```

Method:

- Uses `WasmGrayScott.u_view()` as the source field.
- Converts the `Float32Array` field view into an RGBA `Uint8ClampedArray` pixel
  buffer, matching the data layout expected by browser `ImageData`.
- Compares reusing a pixel buffer against allocating a fresh pixel buffer each
  frame.

Observed output:

| Grid | Cells | Trials | Reuse buffer ms/frame | Allocate buffer ms/frame | Allocation overhead | Reuse checksum | Allocate checksum |
|---|---:|---:|---:|---:|---:|---:|---:|
| 128x128 | 16384 | 1000 | 0.054530 | 0.056977 | 1.04x | 16320000 | 16320000 |
| 256x256 | 65536 | 1000 | 0.206754 | 0.213902 | 1.03x | 16320000 | 16320000 |
| 512x512 | 262144 | 1000 | 0.828732 | 0.848691 | 1.02x | 16320000 | 16320000 |

Interpretation:

- Field-to-RGBA conversion is linear in cell count and remains under `1 ms/frame`
  at `512 x 512` in Node.js.
- Reusing a pixel buffer is slightly faster, but allocation overhead is small in
  this benchmark.
- Browser rendering still needs to measure `ImageData` construction and
  `putImageData`/OffscreenCanvas costs separately.

---

## Multi-Regime Full-Field Validation

Commands:

```bash
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000 --feed 0.037 --kill 0.060
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000 --feed 0.060 --kill 0.062
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000 --feed 0.025 --kill 0.060
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000 --feed 0.050 --kill 0.065
```

Observed output:

| Grid | Steps | Regime | u_MAE | v_MAE | u_RMSE | v_RMSE | u_MaxErr | v_MaxErr |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| 64x64 | 100 | F=0.037, k=0.060 | 2.467e-08 | 1.702e-09 | 4.218e-08 | 8.662e-09 | 1.788e-07 | 8.941e-08 |
| 64x64 | 500 | F=0.037, k=0.060 | 5.736e-08 | 2.364e-08 | 1.066e-07 | 6.745e-08 | 7.749e-07 | 6.258e-07 |
| 64x64 | 1000 | F=0.037, k=0.060 | 1.697e-07 | 1.002e-07 | 2.714e-07 | 1.794e-07 | 1.311e-06 | 9.537e-07 |
| 64x64 | 100 | F=0.060, k=0.062 | 2.184e-08 | 2.080e-09 | 4.181e-08 | 1.401e-08 | 2.384e-07 | 1.937e-07 |
| 64x64 | 500 | F=0.060, k=0.062 | 3.744e-08 | 1.014e-08 | 6.996e-08 | 3.951e-08 | 5.364e-07 | 3.725e-07 |
| 64x64 | 1000 | F=0.060, k=0.062 | 4.238e-08 | 1.755e-08 | 8.347e-08 | 5.602e-08 | 5.960e-07 | 5.364e-07 |
| 64x64 | 100 | F=0.025, k=0.060 | 3.746e-08 | 5.244e-09 | 6.252e-08 | 2.536e-08 | 4.172e-07 | 2.980e-07 |
| 64x64 | 500 | F=0.025, k=0.060 | 5.345e-08 | 1.890e-08 | 9.784e-08 | 5.985e-08 | 6.557e-07 | 5.364e-07 |
| 64x64 | 1000 | F=0.025, k=0.060 | 1.387e-07 | 5.978e-08 | 2.560e-07 | 1.432e-07 | 1.431e-06 | 9.984e-07 |
| 64x64 | 100 | F=0.050, k=0.065 | 2.036e-08 | 2.540e-09 | 3.994e-08 | 1.584e-08 | 2.980e-07 | 2.235e-07 |
| 64x64 | 500 | F=0.050, k=0.065 | 5.822e-08 | 2.027e-08 | 1.782e-07 | 1.363e-07 | 1.907e-06 | 1.788e-06 |
| 64x64 | 1000 | F=0.050, k=0.065 | 6.430e-07 | 3.995e-07 | 3.380e-06 | 2.707e-06 | 3.833e-05 | 3.499e-05 |

Interpretation:

- All tested regimes match the NumPy `float32` reference at small absolute error.
- The `F=0.050, k=0.065` regime accumulates more error by `1000` steps than the
  others. This should be treated as a sensitive regime in later validation, not
  ignored.
- The larger `1000`-step error is still small in absolute terms, with max error
  below `4e-5`.

---

## Quality Gate

Local quality command:

```bash
bash tools/quality.sh
```

Pre-commit setup:

```bash
bash tools/install-hooks.sh
```

Manual pre-commit run:

```bash
PRE_COMMIT_HOME=.pre-commit-cache \
.venv/bin/pre-commit run --all-files
```

Checks included:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `ruff format --check .`
- `ruff check .`
- Rust vs scalar Python summary comparison
- Rust vs NumPy `float32` summary comparison
- Rust vs NumPy full-field metrics for `64 x 64`, `100/500/1000` steps

CI:

- `.github/workflows/quality.yml` runs the same quality gate on push and pull
  request.

Git hooks:

- `.pre-commit-config.yaml` defines a local `quality-gate` hook that runs
  `bash tools/quality.sh`.
- `tools/install-hooks.sh` installs the hook for both `pre-commit` and `pre-push`.
