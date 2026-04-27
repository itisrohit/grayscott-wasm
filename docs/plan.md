# PLAN.md - Defensible Research Plan: Differentiable Gray-Scott in Rust/WASM

This is the corrected version of the plan after venue and literature checks.

Blunt verdict: this is not defensible as "the first differentiable PDE solver" or
"the first browser PDE simulator." Those claims are false or too fragile. The
defensible version is a focused systems paper: a reproducible, browser-deployable,
CPU-only WebAssembly implementation of a differentiable Gray-Scott solver, with
measured accuracy, performance, AD overhead, inverse-recovery behavior, and failure
modes.

The project is still worth doing, but only if the paper is framed as a careful
benchmark/design study rather than a breakthrough numerical-method paper.

---

## 0. What Is The Actual Research Claim?

Weak claim to avoid:

> "No one has done differentiable PDE solving before."

This is not credible. JAX-Fluids, PhiFlow, JAX-FEM, and related differentiable
physics tools already exist.

Weak claim to avoid:

> "No one has run reaction-diffusion in a browser before."

This is also not credible. VisualPDE runs interactive PDE simulations in the
browser, including reaction-diffusion systems such as Gray-Scott.

Defensible claim:

> "We present and evaluate a Rust/WebAssembly implementation of a differentiable
> Gray-Scott reaction-diffusion solver for small-parameter inverse recovery in the
> browser. The study quantifies forward-solver accuracy, scalar Rust/WASM/JS performance,
> forward-mode automatic differentiation overhead, gradient correctness, and inverse
> recovery robustness under initialization and noise."

The word "evaluate" matters. The contribution is not just code; it is the measured
tradeoff profile of using WebAssembly and forward-mode AD for a browser-deployable
inverse PDE task.

---

## 1. Why This Could Still Be Publishable

The credible gap is a deployment and tradeoff gap:

- Differentiable PDE frameworks such as JAX-Fluids and PhiFlow are powerful, but
  they are Python-centered and typically depend on ML runtimes for automatic
  differentiation.
- Browser PDE tools such as VisualPDE demonstrate interactive PDE simulation, but
  they are not primarily framed as automatic-differentiation inverse solvers.
- Rust/WASM can package compute kernels into portable browser modules, but the
  cost/accuracy tradeoffs for differentiating a reaction-diffusion inverse problem
  in WASM are not well characterized.

So the paper should not say:

- "first ever"
- "beats JAX"
- "GPU-free replacement for differentiable physics"
- "solves inverse pattern recovery generally"

It can say:

- "browser-deployable"
- "CPU-only"
- "small-parameter inverse recovery"
- "forward-mode AD is appropriate for two scalar parameters"
- "we quantify where this approach works and where it fails"

That is a much stronger defense because it does not collapse when a reviewer points
to existing differentiable PDE systems.

---

## 2. Working Title And Venue

Working title:

> RD-WASM: A Browser-Deployable Differentiable Gray-Scott Solver in Rust and
> WebAssembly

Better subtitle if needed:

> Accuracy, Performance, and Failure Modes of Forward-Mode AD for In-Browser
> Reaction-Diffusion Parameter Recovery

Primary venue:

- IEEE Access, only if the experiments are complete and the APC can be paid or
  waived through an institutional agreement.

Important correction:

- IEEE Access is not free. IEEE's 2026 APC page lists IEEE Access at USD 2,160,
  plus possible taxes. Student discounts do not apply. Check institutional open
  access agreements before assuming submission is financially possible.

Alternative venues to consider:

- SoftwareX, if the artifact becomes the strongest contribution.
- Journal of Open Source Software (JOSS), if the goal becomes a software paper
  rather than a full systems/research article.
- A workshop or student research track, if the final inverse results are too weak
  for a journal.

Do not submit until the result tables are real. A paper plan without data is not a
paper.

---

## 2.1 Current Artifact Status

Current paper status:

- IEEEtran paper source exists at `paper/main.tex`.
- Bibliography exists at `paper/references.bib`.
- Figures live in `paper/figures/`.
- A compiled Overleaf PDF has been added locally at
  `paper/grayscott_wasm_IEEE_Journal_Paper.pdf`.
- The current paper scope is scalar Rust/WASM plus forward-mode AD. SIMD is
  explicitly future work, not a current contribution.
- The repository URL is included in the paper:
  `https://github.com/itisrohit/grayscott-wasm`.

As of the current implementation, the project has moved past planning for the
forward solver. The following pieces exist and are measured:

- Native Rust scalar solver.
- Dependency-free Python scalar reference.
- NumPy `float32` reference.
- Node.js scalar JavaScript benchmark.
- Node.js scalar WASM benchmark via `wasm-pack`.
- Rust-vs-NumPy full-field validation.
- WASM-vs-NumPy full-field validation.
- Browser WASM package build for real browser measurements.
- Initial inverse-recovery grid-search baseline for `F` and `k`.
- Multi-grid and multi-regime correctness logs.
- Local and CI quality gates:
  - `cargo fmt --check`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`
  - `ruff format --check`
  - `ruff check`
  - JavaScript syntax checks
  - Node.js and browser WASM builds
  - Node.js WASM checks
  - Rust/NumPy/WASM validation scripts

Important measured result:

- Scalar WASM is only about `1.22x-1.27x` faster than scalar JavaScript in the
  current Node.js benchmark.
- Native Rust is still about `1.31x-1.49x` faster than scalar WASM.
- Therefore, the paper does not claim a large scalar WASM speedup. SIMD
  acceleration of the Laplacian kernel is left to future work.

Current quality command:

```bash
PRE_COMMIT_HOME=.pre-commit-cache .venv/bin/pre-commit run --all-files
```

Current WASM build command:

```bash
bash tools/build_wasm_node.sh
bash tools/build_wasm_web.sh
```

Current browser rendering benchmark:

```bash
python3 -m http.server 8000
```

Open `http://localhost:8000/www/render_bench.html` after building the browser
WASM package. The harness records field-to-RGBA conversion, `ImageData`
construction, `putImageData`, and optional OffscreenCanvas/ImageBitmap timings.
Initial Chrome manual measurements are now recorded in `docs/experiment-log.md`.
The current result says field-to-RGBA conversion is the dominant render-side cost
at `512 x 512` (`0.817000 ms/frame` median), while direct `putImageData` is below
`0.1 ms/frame` in that environment. This is useful but still a single-browser,
single-machine result, so the paper should qualify any browser-rendering claim
until another browser or machine is measured.

Current inverse baseline command:

```bash
cargo run --release --bin inverse_grid -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --feed-min 0.058 --feed-max 0.063 --feed-count 11 \
  --kill-min 0.060 --kill-max 0.065 --kill-count 11
```

Current multi-regime inverse recovery command:

```bash
cargo run --release --bin inverse_regimes -- \
  --width 64 --height 64 --steps 100 \
  --feed-min 0.045 --feed-max 0.070 --feed-count 51 \
  --kill-min 0.055 --kill-max 0.070 --kill-count 31
```

Current noise-sensitivity command:

```bash
cargo run --release --bin inverse_noise -- \
  --width 64 --height 64 --steps 100 \
  --noise-levels 0.000,0.001,0.005,0.010,0.020,0.050,0.100 \
  --seeds 24301,24589,51966,48879 \
  --feed-min 0.045 --feed-max 0.070 --feed-count 51 \
  --kill-min 0.055 --kill-max 0.070 --kill-count 31
```

Current grid-search vs AD-optimizer command:

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

This is a baseline, not the final differentiable method. It gives the paper a
clear recovery target that finite-difference gradients and forward-mode AD must
match or improve.

Current finite-difference gradient command:

```bash
cargo run --release --bin inverse_grad -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Current finite-difference optimizer command:

```bash
cargo run --release --bin inverse_opt -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --initial-feed 0.060 --initial-kill 0.063 \
  --learning-rate 0.0001 --epsilon 0.0001 --iterations 8
```

Current forward-mode AD gradient comparison command:

```bash
cargo run --release --bin inverse_ad -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Current inverse-gradient overhead benchmark command:

```bash
cargo run --release --bin bench_inverse -- \
  --grids 64,128,256 --steps 100 --trials 7 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Current statistical timing command:

```bash
cargo bench --bench inverse_overhead
```

Current inverse result:

- A `64 x 64`, 100-step target with `F = 0.060` and `k = 0.062` is recovered
  exactly when the target pair is present in a 5-by-5 candidate grid.
- An off-grid `64 x 64`, 100-step target with `F = 0.06055` and `k = 0.06245`
  is recovered to the nearest 11-by-11 grid candidate, with absolute parameter
  errors of `0.00005` for both `F` and `k`.
- A three-regime grid-search check recovers close parameters across tested
  regimes, but the lower-feed case has larger `F` error (`0.000750`) despite low
  field loss. This supports the existing caution that inverse recovery should be
  reported with both field loss and parameter error.
- A four-seed deterministic noise sweep on the default off-grid target keeps the
  same recovered candidate through uniform noise amplitude `0.020`, starts mixed
  failures at `0.050`, and shows clearer degradation at `0.100`.
- Central finite differences now estimate the loss gradient with respect to `F`
  and `k`; this becomes the comparison target for forward-mode AD.
- Fixed-step finite-difference gradient descent reduces pattern loss from
  `3.761e-5` to `1.317e-5` over 8 iterations from the current off-target guess.
- Forward-mode AD now matches central finite differences on the same case with
  relative gradient deltas below `4e-4`.
- In native overhead measurements at `128x128` and `256x256`, finite differences
  cost about `5.0x` a primal loss evaluation, while forward-mode AD costs about
  `2.6x`.
- The `64x64` overhead ratio is not treated as stable because the small timing
  scale is noisy. The defensible Experiment 4 claim is the larger-grid result:
  forward-mode AD is roughly `2x` cheaper than finite differences per
  two-parameter gradient query.
- Criterion is now the preferred timing harness for this overhead result because
  it applies warmup, repeated sampling, outlier handling, and confidence
  intervals. The `bench_inverse` binary remains useful only as a fast rough
  check.
- The `2.6x` AD overhead is likely dominated partly by memory traffic/cache
  pressure from carrying value plus two derivatives per field cell, not just
  arithmetic. This belongs in the discussion and motivates adjoint/reverse-mode
  methods for larger parameter counts.
- The optimizer result also shows that lower pattern loss does not necessarily
  mean smaller raw parameter distance to the generating parameters, so inverse
  recovery claims must report both loss and parameter error.
- A fixed-step AD optimizer runs against the same noise cases. It uses only `9`
  evaluations per case versus `1581` grid candidates, stays stable across the
  tested seeds, but does not beat dense grid search on clean/noisy loss.
- An Armijo backtracking AD optimizer now improves that result: it uses `17`
  evaluations per case, reaches lower clean loss than dense grid search on the
  clean target (`1.740e-7` vs `2.997e-7`), and stays competitive with grid-search
  noisy loss through noise amplitude `0.100`.
- Backtracking uses the standard sufficient-decrease rule with shrink `0.5` and
  Armijo constant `1e-4`; these are conventional conservative defaults for
  gradient descent line search.
- These validate the inverse-recovery harness enough for the current paper
  scope. Do not add multi-start AD, longer rollouts, or SIMD before submission
  unless PDF review exposes a concrete gap.
- The paper draft now reports scalar performance, full-field correctness,
  multi-grid correctness, browser rendering, gradient correctness, AD overhead,
  inverse recovery, and noise sensitivity.

---

## 3. Prior Art And How To Position Against It

### Differentiable PDE / Physics Systems

JAX-Fluids:

- Python/JAX-based fully differentiable CFD solver.
- Stronger numerics and broader physical scope than this project.
- Runs on CPU/GPU/TPU through JAX.
- Our positioning: not a competitor; compare only as a Python/AD baseline for the
  same simplified Gray-Scott task if implemented.

PhiFlow:

- Python simulation toolkit for optimization and machine learning.
- Uses NumPy, TensorFlow, JAX, or PyTorch backends.
- Our positioning: shows differentiable PDE simulation already exists; our
  contribution is browser-deployable WASM packaging and measured small-parameter
  forward-mode behavior.

JAX-FEM and related JAX differentiable simulation tools:

- Show that inverse design and AD-based physical simulation are active, mature
  areas.
- Our positioning: this project is not about new AD theory; it is about a small,
  reproducible WebAssembly deployment target.

### Browser PDE / Reaction-Diffusion Systems

VisualPDE:

- Browser-based interactive PDE simulation platform.
- Includes reaction-diffusion systems, including Gray-Scott.
- Our positioning: VisualPDE is important prior browser PDE work. This project must
  distinguish itself through automatic differentiation, inverse recovery, gradient
  checks, and WASM/Rust implementation measurements.

Existing Gray-Scott demos:

- Many JS/WebGL/Canvas demos exist.
- Our positioning: they are prior art for visualization and interaction, not for a
  validated differentiable inverse solver.

### Rust AD / Dual Numbers

Rust crates such as `num-dual`, `numdiff`, and other dual-number libraries already
exist.

Our positioning:

- Do not claim novelty for dual numbers.
- Use a custom minimal dual type only if it makes WASM memory layout and benchmark
  control easier.
- Otherwise, cite the crate ecosystem and explain why the implementation is kept
  small and audit-friendly.

---

## 4. Paper Contributions

Use these contributions, not the old overclaiming ones.

### C1 - Browser-Deployable Solver With Reproducible Benchmarks

A Rust implementation of the Gray-Scott finite-difference solver compiled to
WebAssembly, with scalar WASM benchmarked against JavaScript and native Rust
reference implementations across multiple grid sizes. SIMD is not claimed in the
current paper.

What must be measured:

- accuracy against a reference implementation,
- steps per second,
- memory usage,
- WASM binary size,
- browser and Node.js runtime differences,
- SIMD vs scalar speedup.

### C2 - Forward-Mode AD For Two-Parameter Inverse Recovery

A forward-mode dual-number implementation that differentiates the final pattern
loss with respect to feed rate `F` and kill rate `k`.

What must be measured:

- gradient correctness against central finite differences,
- AD overhead relative to the primal solver,
- inverse recovery accuracy,
- sensitivity to initialization,
- sensitivity to target noise,
- failure cases.

### C3 - Boundary Of Applicability

A documented failure analysis showing when this approach is not appropriate:

- far initialization,
- noisy targets,
- ambiguous patterns,
- long rollout instability,
- more than a small number of parameters,
- spatially varying parameter fields.

This contribution is important. Honest negative results make the paper more
credible.

---

## 5. Research Questions

The paper should answer these questions with data:

RQ1. Accuracy:

- Can the Rust/WASM solver reproduce a carefully controlled reference
  implementation within acceptable numerical tolerance?

RQ2. Performance:

- How much faster is Rust/WASM than a comparable JavaScript implementation?
- How much does explicit WASM SIMD help?
- How much performance is lost through JS/WASM boundary crossings?

RQ3. Differentiability:

- Do dual-number gradients match central finite-difference gradients over multiple
  `F,k` points and rollout lengths?

RQ4. Inverse recovery:

- Under what initializations and noise levels can the method recover `F,k`?
- When does it fail?

RQ5. Deployment:

- Is the complete inverse loop practical in a browser without GPU, Python, or a
  server?

---

## 6. Technical Design Corrections

### Memory Layout

Use separate arrays, not an array-of-structs, unless benchmarks prove otherwise:

```text
u: Vec<f32>
v: Vec<f32>
next_u: Vec<f32>
next_v: Vec<f32>
```

For dual mode, prefer structure-of-arrays:

```text
u: Vec<f32>
u_dF: Vec<f32>
u_dk: Vec<f32>
v: Vec<f32>
v_dF: Vec<f32>
v_dk: Vec<f32>
```

Reason: SIMD and cache behavior are usually better with contiguous arrays than with
`Vec<Dual>`.

### WASM Memory Access

Do not claim `SharedArrayBuffer` unless using threads and serving with the required
cross-origin isolation headers. The simple path is:

```text
WebAssembly.Memory.buffer -> Float32Array view
```

Use `SharedArrayBuffer` only as an optional threaded extension.

Current implementation note:

- `WasmGrayScott` exposes `u_values()` and `v_values()` for validation.
- Those functions copy the fields from WASM into JavaScript-owned arrays.
- That is acceptable for correctness/export scripts, but it is not the right
  browser rendering API.
- The zero-copy accessors now exist:

```text
u_ptr() -> *const f32
v_ptr() -> *const f32
u_view() -> Float32Array
v_view() -> Float32Array
```

JavaScript can either use `u_view()`/`v_view()` directly or create typed-array
views into WASM memory:

```text
Float32Array(wasm.memory.buffer, ptr, len)
```

This follows the Rust/WASM guidance to keep large, long-lived data in WASM memory
and avoid copying/serialization across the JS/WASM boundary.

Safety note:

- Typed-array views into WASM memory are invalidated if the memory grows.
- Browser rendering code should recreate views after any operation that may
  allocate or grow memory.

### SIMD Future Work

SIMD is deferred. If it is added after the current paper, do not assume
`#[target_feature(enable = "simd128")]` is enough. For Rust/WASM:

- keep scalar and SIMD implementations separate,
- gate SIMD with `#[cfg(target_feature = "simd128")]`,
- benchmark both builds,
- handle boundary rows/columns separately,
- verify scalar and SIMD outputs against each other.

### Boundary Conditions

Use periodic boundaries for the paper unless there is a reason not to. They avoid
edge artifacts and simplify comparisons. But do not hide the boundary choice; state
it in every experiment.

### Numerical Precision

Use `f32` consistently in both Rust and reference code when testing WASM parity.
If NumPy is used, force `np.float32`. Also report:

- MAE,
- RMSE,
- max absolute error,
- optional SSIM or correlation for pattern similarity.

Do not promise `MAE < 1e-5` until measured.

---

## 7. Experiments Required For A Credible Paper

### Experiment 1 - Forward Solver Correctness

Goal:

- Prove that the implementation is numerically sane.

Setup:

- grids: 128x128, 256x256, 512x512,
- steps: 100, 500, 1000,
- parameters: at least four Pearson-style regimes,
- same initial condition saved to disk,
- `f32` reference implementation.

Metrics:

- MAE,
- RMSE,
- max absolute error,
- visual pattern comparison.

Acceptance gate:

- Scalar Rust native, Rust/WASM, and reference implementation must agree within a
  measured and justified tolerance.

### Experiment 2 - Performance

Goal:

- Quantify browser-deployable performance.

Compare:

- JavaScript scalar implementation,
- Rust/WASM scalar,
- Rust/WASM SIMD,
- native Rust binary if useful,
- NumPy reference as a scientific baseline,
- optional JAX CPU baseline if implemented.

Metrics:

- steps/sec,
- ms/step,
- memory usage,
- binary size,
- startup/initialization cost,
- browser version and hardware.
- JS/WASM boundary mode:
  - bulk `run(steps)`,
  - repeated `step()` calls.

Important:

- Do not compare against JAX-GPU as if this project should win. It should not.
- Do not assume WASM is dramatically faster than JavaScript. Current scalar
  measurements show only modest speedup.

### Experiment 3 - Gradient Correctness

Goal:

- Prove that dual-mode AD computes the correct derivatives.

Setup:

- choose multiple `F,k` points,
- use multiple rollout lengths: 10, 50, 100, 500,
- compute loss against a fixed target,
- compare dual gradients against central finite differences:

```text
dL/dF ~= (L(F + eps, k) - L(F - eps, k)) / (2 eps)
dL/dk ~= (L(F, k + eps) - L(F, k - eps)) / (2 eps)
```

Metrics:

- relative gradient error,
- absolute gradient error,
- runtime ratio vs finite differences.

This experiment is mandatory. Without it, the differentiability claim is weak.

### Experiment 4 - AD Overhead

Goal:

- Measure the cost of forward-mode AD.

Compare:

- primal solver,
- dual solver for `F,k`,
- central finite difference requiring four extra forward simulations.

Metrics:

- runtime ratio,
- memory ratio,
- browser responsiveness,
- grid-size scaling.

Expected:

- Dual mode will be slower than primal. That is fine.
- The real question is whether it is faster or cleaner than finite differences for
  two parameters.

### Experiment 5 - Inverse Recovery

Goal:

- Test whether `F,k` can be recovered from target patterns.

Setup:

- generate targets from known parameters,
- use held-out initial conditions,
- test multiple random seeds,
- test target noise levels: 0%, 1%, 5%,
- test near and far initializations.

Metrics:

- final `|F_hat - F_true|`,
- final `|k_hat - k_true|`,
- final loss,
- success rate over seeds,
- number of optimizer iterations,
- wall-clock time.

Baselines:

- finite-difference gradient descent,
- random search or grid search over a bounded parameter box,
- optional JAX/Autograd baseline if available.

Important:

- A same-solver synthetic target is not enough. Add noise and seed variation.
- Show failures. A reviewer will expect them.

### Experiment 6 - Browser Deployment

Goal:

- Demonstrate that the full inverse loop actually runs in-browser.

Metrics:

- time per inverse iteration,
- UI frame responsiveness if visualization is enabled,
- memory use,
- largest practical grid size.

Implementation:

- run solver in a Web Worker,
- send only compact progress data to the main thread,
- render `u` as an image buffer.

---

## 8. Paper Structure

### I. Introduction

- Motivate reaction-diffusion and inverse parameter recovery.
- State that differentiable PDE tools exist but are typically Python/ML-runtime
  centered.
- State that browser PDE demos exist but are not usually evaluated as
  differentiable inverse solvers.
- Present this paper as a measured WebAssembly/Rust design study.
- List C1, C2, C3.

### II. Background And Related Work

Cover:

- Turing patterns,
- Gray-Scott model,
- Pearson parameter regimes,
- finite-difference reaction-diffusion,
- WebAssembly and Rust/WASM tooling,
- WASM SIMD,
- forward-mode AD and dual numbers,
- differentiable PDE systems,
- browser PDE systems.

Do not bury related work. The paper survives only if it shows awareness of existing
systems.

### III. System Design

Explain:

- memory layout,
- time stepping,
- boundary conditions,
- scalar Rust/WASM kernels,
- WASM interface,
- browser render path and zero-copy field views,
- dual-mode derivative propagation,
- inverse optimization loop.

### IV. Validation And Experiments

This should be the largest section.

Include all six experiments above, or at least the first five. Experiment 6 can be
folded into the demo/deployment section if space is tight.

### V. Discussion

Explain:

- where WASM helps,
- where JAX/Python tools are better,
- why forward-mode is reasonable for two parameters,
- why it does not scale to many parameters,
- how browser deployment changes reproducibility and accessibility.

### VI. Limitations

State directly:

- no new numerical scheme,
- no GPU comparison win,
- no guarantee of global inverse recovery,
- no spatially varying parameter recovery,
- explicit Euler stability limits,
- browser performance varies by engine and hardware.

### VII. Conclusion

Restate measured findings only. Do not claim general superiority.

---

## 9. Implementation Plan

Repository structure:

```text
grayscott-wasm/
  Cargo.toml
  src/
    lib.rs
    solver.rs
    inverse.rs
  www/
    index.html
    main.js
    worker.js
  benches/
    inverse_overhead.rs
  reference/
    scalar_reference.py
    numpy_reference.py
  docs/
    experiment-log.md
    manualcheck-browser-render.md
    plan.md
  tools/
    bench_forward_js.mjs
    bench_forward_wasm.mjs
    bench_grayscale_render.mjs
    full_field_metrics.py
    wasm_full_field_metrics.py
  paper/
    figures/
    main.tex
    references.bib
  README.md
```

Phases:

1. Build scalar Rust solver and NumPy/JS references.
2. Validate scalar correctness with saved initial conditions.
3. Add WASM build and browser rendering.
4. Add scalar benchmark harness.
5. Add dual-mode derivative propagation.
6. Add finite-difference gradient checks.
7. Add inverse optimizer and recovery experiments.
8. Add noise/seed robustness experiments.
9. Write the paper only after the result tables exist.
10. Compile and archive the paper PDF.

Current status: steps 1--10 are complete for the scalar Rust/WASM paper scope.
The old SIMD step is no longer a prerequisite for this paper and is now future
work.

---

## 10. Realistic Timeline

Assuming part-time college workload:

| Week | Goal |
|---|---|
| 1 | Scalar solver, reference implementations, saved initial conditions |
| 2 | Correctness validation and basic browser WASM demo |
| 3 | Benchmark harnesses, JS/WASM/native comparisons |
| 4 | Browser rendering and scalar performance measurements |
| 5 | Dual-mode AD implementation |
| 6 | Gradient correctness experiment |
| 7 | Inverse optimizer and synthetic recovery |
| 8 | Robustness tests: noise, seeds, far initialization |
| 9 | Figures, tables, reproducibility scripts |
| 10 | Paper draft |
| 11 | Revision, related work cleanup, artifact documentation |
| 12 | Compiled PDF and final submission decision |

Seven weeks was too optimistic. Ten to twelve weeks is more realistic if the work is
done properly.

---

## 11. Rejection Risks And Defenses

### Risk: "Insufficient novelty"

Defense:

- Do not claim new AD theory or new PDE numerics.
- Position as a reproducible systems evaluation of browser-deployable
  differentiable reaction-diffusion.
- Include strong related work and measured tradeoffs.

### Risk: "Toy problem"

Defense:

- Add noise, seed variation, and failure analysis.
- Compare against finite differences and grid/random search.
- Make clear that Gray-Scott is a controlled benchmark, not a claim of general
  scientific inverse modeling.

### Risk: "WASM is slower than native/JAX"

Defense:

- That is acceptable. The goal is portability and browser deployment.
- Report native/JAX wins honestly.

### Risk: "Forward-mode does not scale"

Defense:

- Correct. The paper is explicitly about two scalar parameters.
- State that adjoint/reverse-mode methods are required for high-dimensional
  parameter fields.

### Risk: "Browser results are hardware-dependent"

Defense:

- Report hardware, browser version, OS, CPU, and repeated trials.
- Publish scripts and raw data.

---

## 12. Sources Checked For This Revision

Venue and cost:

- IEEE Access author information:
  https://ieeeaccess.ieee.org/authors/
- IEEE Access about page:
  https://ieeeaccess.ieee.org/about/
- IEEE Open 2026 APC page:
  https://open.ieee.org/for-authors/article-processing-charges/

WebAssembly and Rust/WASM:

- WebAssembly specifications:
  https://webassembly.org/specs
- Haas et al., "Bringing the Web up to Speed with WebAssembly," PLDI 2017:
  https://pldi17.sigplan.org/details/pldi-2017-papers/48/Bringing-the-Web-up-to-Speed-with-WebAssembly
- MDN Rust to WebAssembly guide:
  https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm
- Rust `wasm32-unknown-unknown` target documentation:
  https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
- MDN WebAssembly `v128` / SIMD reference:
  https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Types/v128
- wasm-pack build docs:
  https://rustwasm.github.io/docs/wasm-pack/commands/build.html
- wasm-bindgen guide:
  https://rustwasm.github.io/docs/wasm-bindgen/

Differentiable PDE / physics:

- JAX-Fluids:
  https://www.sciencedirect.com/science/article/pii/S0010465522002466
- JAX-Fluids 2.0:
  https://www.sciencedirect.com/science/article/pii/S0010465524003564
- PhiFlow:
  https://pypi.org/project/phiflow/

Browser PDE / reaction-diffusion:

- VisualPDE:
  https://link.springer.com/article/10.1007/s11538-023-01218-4
- Example Gray-Scott browser/demo explanation:
  https://www.4rknova.com/blog/2026/02/15/reaction-diffusion

Gray-Scott and inverse-problem context:

- High-fidelity Gray-Scott simulations:
  https://www.sciencedirect.com/science/article/pii/S0096300323002485
- Recent Gray-Scott numerical schemes:
  https://www.sciencedirect.com/science/article/abs/pii/S0168927426000012
- Reaction-advection-diffusion inverse problems:
  https://www.sciencedirect.com/science/article/pii/S0377042724007027
- Reaction-diffusion parameter estimation example:
  https://pmc.ncbi.nlm.nih.gov/articles/PMC4643433/

Rust dual-number / AD ecosystem:

- `num-dual`:
  https://docs.rs/num-dual
- `numdiff`:
  https://docs.rs/numdiff

---

## 13. First Concrete Next Step

The original implementation plan is complete for the current scalar Rust/WASM
paper scope: scalar Rust solver, references, validation, native benchmarks, JS
benchmark, scalar WASM benchmark, browser-render measurements, quality gates,
forward-mode AD, inverse recovery, noise sensitivity, and paper draft all exist.

Immediate next task:

1. Decide whether to commit the compiled PDF
   `paper/grayscott_wasm_IEEE_Journal_Paper.pdf`.
2. Create a reproducibility tag after the PDF and plan update are committed, for
   example `paper-draft-v1`.
3. Use that tag when referencing the artifact in submissions or preprints.

Future work, not a blocker for this paper:

- implement a separate WASM SIMD Laplacian kernel,
- validate scalar/SIMD parity,
- measure scalar-vs-SIMD speedup,
- repeat browser measurements across more browsers and machines.
