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
