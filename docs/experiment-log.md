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

- MAE over full `u` field
- MAE over full `v` field
- RMSE over full fields
- Max absolute error
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

Not yet done:

- Full-field Rust vs NumPy MAE/RMSE/max-error comparison.
- Multi-grid validation: `128 x 128`, `256 x 256`, `512 x 512`.
- Multi-step validation: `100`, `500`, `1000`.
- Multiple parameter regimes.
- Native Rust performance benchmark.
- JavaScript baseline.
- WASM build.
- SIMD build.
- Automatic differentiation.
- Gradient checks.
- Inverse recovery.

---

## Next Validation Upgrade

The next correctness milestone should compare full fields, not only summary stats.

Required output:

| Grid | Steps | Regime | u_MAE | v_MAE | u_RMSE | v_RMSE | u_MaxErr | v_MaxErr |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| 64x64 | 100 | F=0.060, k=0.062 | TBD | TBD | TBD | TBD | TBD | TBD |
| 64x64 | 500 | F=0.060, k=0.062 | TBD | TBD | TBD | TBD | TBD | TBD |
| 64x64 | 1000 | F=0.060, k=0.062 | TBD | TBD | TBD | TBD | TBD | TBD |

Implementation note:

- Export Rust final fields to a binary or `.npy`-compatible format.
- Compare against NumPy final fields directly.
- Keep identical initial conditions and `float32` references.
