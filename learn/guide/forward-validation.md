---
sidebar_position: 7
title: Forward Validation and Performance
---

# Forward Validation and Performance

If you want the solver mechanics first, read
[Forward Solver Math](./forward-solver-math) before this page. This page is
about evidence and measurement, not the derivation itself.

## The question on this page

This page answers:

> Can we trust the forward solver, and after that, what do the forward
> performance experiments actually mean?

It combines two experiment families on purpose:

- correctness experiments, because speed without trust is useless;
- forward-runtime experiments, because different runtime paths answer different
  performance questions.

## Part 1: Forward correctness

## Why validate first?

Performance numbers are meaningless if the solver is numerically wrong.

That is why the artifact validates:

- Rust vs dependency-free Python,
- Rust vs NumPy `float32`,
- Rust vs WASM,
- scalar WASM vs SIMD WASM.

This is one of the strongest parts of the repo: it is not just an optimization
demo.

## What is a reference implementation?

A reference implementation is a version you trust as a comparison point.

In this repo, the Python and NumPy versions play that role for forward
validation.

That does **not** mean they are magically perfect. It means they are useful
independent implementations that help answer this question:

> Does the main Rust solver behave the same way as another implementation of
> the same model?

## Why use full-field metrics instead of only summary stats?

Summary stats like min, max, and mean are useful first checks, but they are too
weak by themselves.

Two different fields can share similar summary values while differing locally.

That is why the later validation upgraded to:

- MAE,
- RMSE,
- max absolute error,
- full exported `u` and `v` field comparison.

This is a stronger test because it asks whether the entire field agrees, not
just a few aggregate numbers.

## What did the validation show?

The full-field comparisons showed very small errors across:

- `64 x 64`, `128 x 128`, `256 x 256`, and `512 x 512`,
- `100`, `500`, and `1000` steps,
- several parameter regimes.

That supports the claim that:

- the Rust implementation is numerically aligned with the references,
- the WASM export preserves the expected behavior,
- the SIMD path matches the scalar path within single-precision tolerance.

Representative multi-grid full-field validation:

| Grid | Steps | u_MAE | v_MAE | u_RMSE | v_RMSE | u_MaxErr | v_MaxErr |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 128x128 | 100 | 5.461e-09 | 5.200e-10 | 2.091e-08 | 7.005e-09 | 2.384e-07 | 1.937e-07 |
| 256x256 | 500 | 2.340e-09 | 6.337e-10 | 1.749e-08 | 9.877e-09 | 5.364e-07 | 3.725e-07 |
| 512x512 | 1000 | 7.876e-10 | 2.742e-10 | 1.101e-08 | 7.002e-09 | 5.960e-07 | 5.364e-07 |

## Why compare multiple regimes?

Because a numerically quiet regime can hide problems.

The multi-regime check exists to answer:

> Does the solver still match the reference when the pattern family changes?

The result was still good, but one regime accumulated more long-run error by
`1000` steps than the others. That does not invalidate the solver. It tells you
where the system is more sensitive and where later experiments should be more
careful.

Representative multi-regime validation:

| Regime | Steps | u_MAE | v_MAE | u_MaxErr | v_MaxErr |
| --- | ---: | ---: | ---: | ---: | ---: |
| F=0.037, k=0.060 | 1000 | 1.697e-07 | 1.002e-07 | 1.311e-06 | 9.537e-07 |
| F=0.060, k=0.062 | 1000 | 4.238e-08 | 1.755e-08 | 5.960e-07 | 5.364e-07 |
| F=0.025, k=0.060 | 1000 | 1.387e-07 | 5.978e-08 | 1.431e-06 | 9.984e-07 |
| F=0.050, k=0.065 | 1000 | 6.430e-07 | 3.995e-07 | 3.833e-05 | 3.499e-05 |

## What does “single-precision tolerance” mean?

The solver uses `f32`, not `f64`.

So you should not expect bit-for-bit identity across every implementation path,
especially once SIMD changes operation ordering. Instead, the right question is:

> Are the differences small enough to be numerically harmless for the intended
> use?

For this project, the answer is yes.

## Why validate WASM separately if Rust already passed?

Because the wrapper path is part of the artifact too.

Compiling Rust to WASM should not change the numerical result, but engineering
mistakes can still happen at the export boundary:

- wrong buffer interpretation,
- wrong data ordering,
- wrong view/copy logic,
- accidental wrapper bugs.

The WASM full-field validation exists to prove that the browser-deliverable path
still matches the reference model.

WASM full-field validation:

| Steps | u_MAE | v_MAE | u_RMSE | v_RMSE | u_MaxErr | v_MaxErr |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 100 | 2.184e-08 | 2.080e-09 | 4.181e-08 | 1.401e-08 | 2.384e-07 | 1.937e-07 |
| 500 | 3.744e-08 | 1.014e-08 | 6.996e-08 | 3.951e-08 | 5.364e-07 | 3.725e-07 |
| 1000 | 4.238e-08 | 1.755e-08 | 8.347e-08 | 5.602e-08 | 5.960e-07 | 5.364e-07 |

This table matters because it closes the loop:

- native Rust matched the reference,
- exported WASM matched the reference too.

So the browser-deliverable path is not a different numerical solver. It is the
same solver seen through a different runtime boundary.

## Part 2: Forward performance

## What did the forward performance experiments compare?

The repo compares:

- native Rust,
- scalar JavaScript,
- scalar WASM,
- SIMD WASM.

This is useful because each pair answers a different question:

- Rust vs WASM:
  What is the browser-delivery cost?
- scalar WASM vs JS:
  Does moving the solver into WASM help at all?
- SIMD WASM vs scalar WASM:
  Does low-level vectorization matter?

## Why compare multiple runtimes at all?

Because each runtime answers a different question:

- **native Rust** shows the direct CPU-side implementation cost,
- **Node.js JavaScript** shows a plain JS baseline,
- **Node.js scalar WASM** isolates runtime and export effects without visible
  browser rendering,
- **Node.js SIMD WASM** shows the effect of explicit vectorization,
- **browser runs** add rendering and UI-facing behavior.

So the repo is not comparing environments randomly. It is separating concerns.

## What did the native, JS, and scalar WASM benchmarks mean?

Those three baselines answer three different questions:

- **native Rust**:
  what does the direct implementation cost on the CPU?
- **JavaScript**:
  what is the plain non-WASM baseline for the same algorithm?
- **scalar WASM**:
  what changes when the Rust solver is delivered as WebAssembly?

The measured story is deliberately modest:

- scalar WASM was only about `1.22x-1.27x` faster than scalar JS,
- native Rust remained faster than scalar WASM by about `1.3x-1.5x`.

Why keep that result? Because it is more credible than pretending the scalar
WASM path delivered a miracle.

Benchmark protocol for the runtime tables:

- release builds,
- same centered-square seed,
- `500` timed steps per trial,
- `5` trials per grid,
- `25` warmup steps before timing.

Forward runtime comparison:

| Grid | Native Rust ms/step | Scalar WASM ms/step | JS ms/step | WASM vs JS | Native vs WASM |
| --- | ---: | ---: | ---: | ---: | ---: |
| 128x128 | 0.050298 | 0.066056 | 0.083996 | 1.27x faster | 1.31x faster |
| 256x256 | 0.176637 | 0.263475 | 0.331804 | 1.26x faster | 1.49x faster |
| 512x512 | 0.707058 | 1.053084 | 1.289701 | 1.22x faster | 1.49x faster |

## What did the boundary-overhead experiment mean?

This experiment asked:

> If JavaScript calls a WASM `step()` repeatedly instead of one `run(steps)`
> call, does the boundary crossing become the real bottleneck?

For the tested grids, the answer was:

- not visibly above timing noise.

That does **not** mean boundary cost is literally zero. It means the per-step
stencil work is large enough that the call boundary did not dominate these
solver timings.

This is why the guide says:

- chunking steps can be chosen for UI responsiveness,
- not because the measured call overhead was large at these grid sizes.

Protocol:

- grids: `64`, `128`, `256`,
- `500` steps,
- `7` trials,
- compare one `run(steps)` call against repeated JS-side `step()` calls.

Boundary-overhead check:

| Grid | Bulk ms/step | Per-step-call ms/step | Boundary overhead |
| --- | ---: | ---: | ---: |
| 64x64 | 0.017932 | 0.017917 | 1.00x |
| 128x128 | 0.068324 | 0.066201 | 0.97x |
| 256x256 | 0.262825 | 0.262816 | 1.00x |

## What did the zero-copy-view experiment mean?

This experiment asked:

> Once the solver is in WASM memory, should the browser path copy fields out or
> read them through typed-array views?

The answer was clear:

- zero-copy views were dramatically faster than copying.

That is why the rendering path prefers `u_view()` and `v_view()` over full
buffer copies.

The tradeoff is that views are more delicate:

- they depend on the current WASM memory buffer,
- they must be recreated if memory grows.

So the design choice is:

- **copy path**: simpler but slower,
- **view path**: faster but requires more careful handling.

Protocol:

- grids: `128`, `256`, `512`,
- `1000` trials,
- compare copied field export against direct typed-array views over WASM memory.

Zero-copy view benchmark:

| Grid | Copy ms/trial | View ms/trial | View speedup |
| --- | ---: | ---: | ---: |
| 128x128 | 0.028776 | 0.001195 | 24.08x |
| 256x256 | 0.033051 | 0.000376 | 87.96x |
| 512x512 | 0.139924 | 0.000249 | 560.82x |

## What did the grayscale render-buffer benchmark mean?

This is still a forward-side experiment even though it looks browser-adjacent.

It asks:

> How expensive is it to turn a numerical field into RGBA pixel bytes before
> the browser upload step even begins?

This is not the same thing as canvas timing.

It isolates:

- field-to-pixel conversion cost,
- buffer reuse vs buffer allocation.

The result was:

- conversion scales roughly linearly with cell count,
- still under `1 ms/frame` at `512 x 512` in the Node path,
- reusing a buffer is slightly better than allocating a fresh one.

This experiment exists so the later browser page can say more precisely where
the rendering cost actually lives.

Protocol:

- grids: `128`, `256`, `512`,
- `1000` trials,
- compare reusing one RGBA buffer against allocating a fresh one per frame.

Render-buffer conversion benchmark:

| Grid | Reuse buffer ms/frame | Allocate buffer ms/frame | Allocation overhead |
| --- | ---: | ---: | ---: |
| 128x128 | 0.054530 | 0.056977 | 1.04x |
| 256x256 | 0.206754 | 0.213902 | 1.03x |
| 512x512 | 0.828732 | 0.848691 | 1.02x |

## What did the SIMD experiment mean?

The SIMD forward experiment asked:

> If we keep the same mathematical solver and change only the execution style,
> does `simd128` materially reduce runtime?

The answer was yes.

The separate SIMD build delivered a large constant-factor win while staying
within single-precision agreement with the scalar path.

Why keep scalar and SIMD separate instead of hiding both in one code path?

- scalar stays readable and easy to trust,
- SIMD stays easy to benchmark directly,
- validation can compare the two paths clearly.

That is better engineering for a research artifact than burying the optimized
path inside conditional branches everywhere.

SIMD validation and speedup:

| Grid | Scalar ms/step | SIMD ms/step | SIMD speedup |
| --- | ---: | ---: | ---: |
| 128x128 | 0.119010 | 0.016002 | 7.44x |
| 256x256 | 0.473616 | 0.055237 | 8.57x |
| 512x512 | 1.910463 | 0.273835 | 6.98x |

Scalar-vs-SIMD validation summary:

- `uMax = 7.153e-7`
- `vMax = 5.364e-7`
- checksum delta `= 5.173e-5`

Protocol:

- Node.js WASM build with `simd128`,
- same `500` steps and `5` trials per grid,
- benchmark scalar `run` against SIMD `run_simd`.

## What was the main forward-performance story?

The important story is not “WASM always beats everything.”

The real story is more careful:

- scalar WASM was only modestly faster than scalar JS in the Node.js path,
- native Rust stayed faster than scalar WASM,
- the dedicated SIMD WASM path produced the biggest improvement.

The runtime story is therefore:

- **algorithm** matters,
- **data export strategy** matters,
- **vectorization** matters,
- plain "compile to WASM" matters, but not nearly as much as people often
  claim.

That is a much more believable systems result than a vague “WASM is fast.”

## What should a student learn from that?

Three things:

1. A language/runtime change by itself may not give huge wins.
2. Memory layout and vectorization can matter more than hype words.
3. Good research explains where speedup came from instead of just reporting one
   big number.

## What does “production-relevant grid size” mean here?

Small toy grids are useful for quick checks, but they can hide the real cost
pattern.

Larger grids are more informative because they stress:

- memory traffic,
- repeated arithmetic,
- cache behavior,
- runtime overheads.

So when the guide says some numbers are more trustworthy at larger grids, it
means those runs behave more like serious simulation work and less like tiny
timing noise.

## Why not lead with browser numbers on this page?

Because browser timing mixes:

- solver cost,
- field export cost,
- pixel conversion,
- canvas upload,
- worker/UI behavior.

This page keeps the forward-side experiments focused on trust and runtime cost.
The browser-specific measurements are separated onto the next experiment page so
the story stays inspectable.
