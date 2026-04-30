# Research Directions For Better Results

This file records internet/literature findings for improving the Gray-Scott
Rust/WASM research artifact. The goal is to avoid random optimization and choose
upgrades that are defensible in a paper.

Current status:

- Scalar Rust, scalar JS, and scalar WASM baselines exist.
- Correctness is validated against NumPy `float32`.
- Scalar WASM is only about `1.22x-1.27x` faster than scalar JS in Node.js for the
  current benchmark.
- A separate WASM SIMD path is now implemented and measured at `6.98x-8.57x`
  over scalar WASM in the Node.js benchmark.
- The browser inverse page now runs the optimizer in `www/inverse_worker.js`.
- The next improvements should target either stronger publication evidence or
  broader browser validation, not basic missing implementation.

---

## Best Next Research Upgrade: Stronger Inverse Problem Math

The current plan uses forward-mode AD for two scalar parameters, `F` and `k`.
That is fine for a browser demo, but it is mathematically modest. State-of-the-art
inverse PDE work usually frames the task as PDE-constrained optimization, often
with adjoint/reduced-space methods, Gauss-Newton methods, regularization, and
explicit noise/observation models.

Relevant evidence:

- Gholami, Mang, and Biros formulate reaction-diffusion parameter estimation as a
  constrained optimization problem and solve it with a Gauss-Newton reduced-space
  algorithm. They explicitly test reconstruction error under noise and detection
  thresholds.
- A 2025 reaction-advection-diffusion inverse-problem paper emphasizes joint
  state-parameter estimation, sparse temporal data, observation operators, and
  separating initial-condition error from measurement noise.

What to add here:

1. Keep forward-mode AD for the first artifact because it is simple and fits two
   scalar parameters.
2. Add finite-difference and grid-search baselines for inverse recovery.
3. Add noisy target experiments and multiple seeds.
4. Add a serious discussion that adjoint/reverse-mode methods are required for
   high-dimensional parameters like `F(x,y)` or unknown initial conditions.
5. Optional stretch: implement adjoint equations or checkpointed reverse-mode for
   parameter recovery. This is a bigger paper, not just a demo.

Why this helps:

- Makes the inverse experiment harder to dismiss as a toy.
- Gives the paper honest alignment with modern inverse-problem literature.
- Opens a future-work path beyond two scalar parameters.

Sources:

- Gholami, Mang, Biros, "An inverse problem formulation for parameter estimation
  of a reaction-diffusion model of low grade gliomas":
  https://pmc.ncbi.nlm.nih.gov/articles/PMC4643433/
- "Joint state-parameter estimation and inverse problems governed by
  reaction-advection-diffusion type PDEs...":
  https://www.sciencedirect.com/science/article/pii/S0377042724007027
- Neural ODE adjoint sensitivity background:
  https://papers.nips.cc/paper/7892-neural-ordinary-differential-equations

---

## Best Next Numerical Upgrade: Higher-Order Or More Stable Schemes

The current solver uses:

- 5-point finite difference Laplacian,
- explicit Euler time integration,
- periodic boundaries.

This is good for a baseline, but it is not state of the art numerically. Recent
Gray-Scott numerical papers use high-fidelity discontinuous Galerkin schemes,
compact high-order difference schemes, stability analysis, and more careful
reaction-term treatment.

Relevant evidence:

- A 2023 high-fidelity Gray-Scott paper uses an explicit mixed modal
  discontinuous Galerkin method on structured meshes and includes stability
  analysis for Turing pattern formation.
- A 2026 Gray-Scott numerical paper develops linearized high-order compact
  difference schemes with second-order temporal and fourth-order spatial accuracy.

What to add here:

1. Keep explicit Euler as the baseline for comparability.
2. Add a second solver mode only after the baseline paper tables are stable.
3. Candidate modes:
   - RK2 or RK4 explicit time integration,
   - semi-implicit diffusion with explicit reaction,
   - compact fourth-order Laplacian,
   - spectral Laplacian for periodic boundaries.
4. Compare accuracy/stability at larger `dt`, not just speed.

Why this helps:

- Better numerical schemes can reduce error or allow larger time steps.
- A "baseline vs improved numerical method" table is more research-relevant than
  only "Rust vs JS".

Risks:

- Higher-order and semi-implicit methods complicate WASM implementation.
- Spectral methods need FFT support and are not ideal unless an FFT dependency is
  justified.
- Changing the numerical method makes exact comparison against the current
  reference harder.

Sources:

- "High-fidelity simulations for Turing pattern formation in multi-dimensional
  Gray-Scott reaction-diffusion system":
  https://www.sciencedirect.com/science/article/pii/S0096300323002485
- "Linearized and high-order accurate one-parameter compact difference schemes
  for solving the Gray-Scott reaction-diffusion model":
  https://www.sciencedirect.com/science/article/abs/pii/S0168927426000012

---

## Performance Upgrade Notes: WASM SIMD Interior Kernel

The measured scalar WASM speedup over JS is modest. A dedicated SIMD kernel for
the interior rows/columns is now implemented, so the focus here is on how to
interpret or extend that result rather than whether to build it.

Use WebAssembly SIMD only after preserving scalar correctness:

- Keep scalar solver as the reference.
- Add a separate WASM SIMD path for interior cells.
- Handle boundaries separately with the scalar periodic path.
- Compare scalar WASM vs SIMD WASM using the same benchmark harness.
- Add scalar/SIMD full-field validation.

Implementation direction:

- Use `core::arch::wasm32` SIMD intrinsics.
- `v128` is the WebAssembly 128-bit SIMD vector type.
- For `f32`, one vector holds four lanes.
- The 5-point stencil can vectorize contiguous interior `x` positions:
  - center: `u[i..i+4]`
  - left: `u[i-1..i+3]`
  - right: `u[i+1..i+5]`
  - up/down: same x range in neighboring rows.

Measured result:

- Do not assume 4x speedup from lane width alone. Memory bandwidth, boundary
  handling, and wasm engine codegen dominate.
- This repo measured `6.98x-8.57x` over scalar WASM in the current Node.js path,
  so any future extension should explain why that number is higher than the
  conservative pre-implementation expectation.

Sources:

- Rust WebAssembly SIMD intrinsics:
  https://doc.rust-lang.org/core/arch/wasm32/index.html
- Rust `v128` type:
  https://doc.rust-lang.org/stable/core/arch/wasm32/struct.v128.html

---

## Browser/UI Performance Upgrade: Worker And Boundary-Crossing Measurement

The current WASM benchmark calls `run(steps)` once per trial. That is good for
kernel throughput, but it hides JS/WASM boundary overhead.

Add two benchmarks:

1. Bulk mode:
   - JS calls `run(500)` once.
   - Measures kernel throughput.

2. Per-step mode:
   - JS calls `step()` 500 times.
   - Measures boundary overhead and UI-style usage.

Why this matters:

- A browser app often needs progress updates and rendering.
- If per-step boundary overhead is high, the UI should run chunks like `run(8)`,
  `run(16)`, or `run(32)` between frames.

Expected result:

- Bulk mode should be faster.
- Per-step mode gives a practical chunk-size recommendation.

Current measured result:

- For `64 x 64`, `128 x 128`, and `256 x 256`, boundary overhead was not visible
  above timing noise when comparing `run(500)` against 500 calls to `step()`.
- This means each Gray-Scott step currently does enough work that JS/WASM call
  overhead is not the bottleneck at these grid sizes.
- This does not eliminate overhead concerns for rendering/data transfer.

State-of-practice guidance from Rust/WASM docs:

- Keep large, long-lived data structures inside WebAssembly memory and expose
  opaque handles to JavaScript.
- Minimize copying across the JS/WASM boundary.
- Avoid serializing/deserializing large fields.
- Return small scalar results for frequent calls.
- For visualization, expose pointers/views into WASM memory instead of returning
  `Vec<f32>` copies every frame.

Action for this repo:

1. Keep `WasmGrayScott` as the opaque long-lived simulation object.
2. Keep `run(steps)` for benchmark and UI chunking.
3. Use zero-copy field access before building a browser renderer:
   - `u_ptr() -> *const f32`
   - `v_ptr() -> *const f32`
   - `u_view() -> Float32Array`
   - `v_view() -> Float32Array`
4. Keep `u_values()` and `v_values()` only for correctness/export scripts because
   they copy.
5. Add a microbenchmark for pure call overhead only if the UI needs very small
   chunk sizes or very small grids.

Current result:

- `u_view()` and `v_view()` match copied fields exactly in the Node.js WASM check.
- View creation plus sampling is substantially cheaper than copying full fields in
  the current benchmark.
- Views must be recreated if WASM memory grows.

Renderer-facing benchmark direction:

- Browser `ImageData.data` uses a `Uint8ClampedArray` in RGBA order.
- `ImageData` is available in Web Workers, so a worker can convert simulation
  fields to pixel buffers before handing rendering work to the main thread or an
  offscreen canvas path.
- Benchmark the conversion from `Float32Array` field view to reusable
  `Uint8ClampedArray` pixel buffer before building UI.
- The repo now includes `www/render_bench.html`, which measures field-to-RGBA
  conversion, `ImageData` construction, `putImageData`, and optional
  OffscreenCanvas/ImageBitmap transfer in a real browser.

Source:

- MDN `ImageData.data`:
  https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data
- MDN `CanvasRenderingContext2D.putImageData`:
  https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData
- MDN `OffscreenCanvas.transferToImageBitmap`:
  https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/transferToImageBitmap
- MDN `ImageBitmapRenderingContext.transferFromImageBitmap`:
  https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferFromImageBitmap

Sources:

- Rust/WASM book interface guidance:
  https://rustwasm.github.io/book/print.html
- Rust/WASM Game of Life tutorial on minimizing copies:
  https://rustwasm.github.io/docs/book/game-of-life/implementing.html
- `js_sys::Float32Array` memory view documentation:
  https://docs.rs/js-sys/latest/js_sys/struct.Float32Array.html

---

## Loss Function Upgrade For Inverse Recovery

Mean squared error on final `u` is simple, but Gray-Scott patterns can be phase
shifted, locally ambiguous, or visually similar with different pixelwise errors.

Add multiple losses:

1. Pixel MSE:
   - current baseline.

2. Multi-scale MSE:
   - compare downsampled fields at several scales.

3. Gradient/Laplacian loss:
   - compare edges/structure, not only concentration values.

4. Spectral loss:
   - compare Fourier magnitude or radial power spectrum.
   - Useful because pattern wavelength matters for Turing patterns.

Why this helps:

- Recovery becomes less brittle to small spatial shifts.
- The paper can report which loss is reliable for which regime.

Risk:

- More complex losses can hide failure if not reported carefully. Always keep MSE
  as a baseline.

---

## Prioritized Implementation Order

Do this next:

1. Add bulk-vs-per-step WASM boundary overhead benchmark.
2. Add scalar WASM full-field output comparison against NumPy/Rust.
3. Add WASM SIMD interior kernel.
4. Add scalar-vs-SIMD correctness and benchmark tables.
5. Add inverse recovery with:
   - finite-difference baseline,
   - grid-search/random-search baseline,
   - noisy targets,
   - multiple seeds.
6. Add better inverse losses only after the baseline inverse recovery works.

Avoid for now:

- Neural PDE solvers/PINNs. They are interesting but would turn this into a
  different project and weaken the clean Rust/WASM systems contribution.
- WebGPU before scalar/SIMD WASM is complete. WebGPU is a different backend and
  changes the research claim.
- Spatially varying `F(x,y)` recovery until adjoint/reverse-mode machinery exists.

---

## Paper Framing Update

The strongest defensible research story is:

> "A measured browser-deployable differentiable reaction-diffusion artifact, with
> correctness validation, scalar JS/Rust/WASM baselines, SIMD acceleration, and
> honest inverse-recovery failure analysis."

Do not frame it as:

> "WASM is automatically much faster than JavaScript."

The current measurements already show that scalar WASM speedup is modest. The
paper should be respected because it measures this honestly.
