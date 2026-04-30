---
sidebar_position: 6
title: Forward Validation and Performance
---

# Forward Validation and Performance

## Why validate first?

Performance numbers are meaningless if the solver is numerically wrong.

That is why the artifact validates:

- Rust vs dependency-free Python,
- Rust vs NumPy `float32`,
- Rust vs WASM,
- scalar WASM vs SIMD WASM.

This is one of the strongest parts of the repo: it is not just an optimization
demo.

## What did the validation show?

The full-field comparisons showed very small errors across multiple grids and
step counts. That supports the claim that:

- the Rust implementation is numerically aligned with the references,
- the WASM export preserves the expected behavior,
- the SIMD path matches the scalar path within single-precision tolerance.

## What does “single-precision tolerance” mean?

The solver uses `f32`, not `f64`.

So you should not expect bit-for-bit identity across every implementation path,
especially once SIMD changes operation ordering. Instead, the right question is:

> Are the differences small enough to be numerically harmless for the intended
> use?

For this project, the answer is yes.

## What did the forward performance experiments compare?

The repo compares:

- native Rust,
- scalar JavaScript,
- scalar WASM,
- SIMD WASM.

This is a useful comparison because each pair answers a different question:

- Rust vs WASM:
  What is the browser-delivery cost?
- scalar WASM vs JS:
  Does moving the solver into WASM help at all?
- SIMD WASM vs scalar WASM:
  Does low-level vectorization matter?

## What was the main forward-performance story?

The important story is not “WASM always beats everything.”

The real story is more careful:

- scalar WASM was only modestly faster than scalar JS in the Node.js path,
- native Rust stayed faster than scalar WASM,
- the dedicated SIMD WASM path produced the biggest improvement.

That is a much more believable systems result than a vague “WASM is fast.”

## What should a student learn from that?

Three things:

1. A language/runtime change by itself may not give huge wins.
2. Memory layout and vectorization can matter more than hype words.
3. Good research explains where speedup came from instead of just reporting one
   big number.
