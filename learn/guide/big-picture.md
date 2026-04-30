---
sidebar_position: 2
title: The Big Picture
---

# The Big Picture

## What is the project really about?

At first glance, this looks like a simulation project. It generates
reaction-diffusion patterns, especially Gray-Scott patterns.

But the real research question is narrower and more interesting:

> How much scientific and inverse-modeling work can a browser-friendly
> Rust/WebAssembly artifact do while still staying reproducible and easy to
> inspect?

That question matters because many serious differentiable-physics tools live in
Python ecosystems and often assume heavier numerical or ML stacks. This project
tests a smaller, CPU-only, browser-deliverable path.

## What the project is not claiming

The artifact does **not** claim:

- to be the first differentiable PDE solver,
- to replace mature differentiable physics frameworks,
- to solve large inverse problems,
- to outperform GPU-based systems.

This matters because good research writing is not just about what you did. It is
also about being precise about what you did **not** do.

## Why Gray-Scott?

Gray-Scott is a strong teaching and systems benchmark because:

- the equations are compact,
- the patterns are visually meaningful,
- the solver is small enough to run in a browser,
- the parameters `F` and `k` actually change the final pattern in visible ways.

That makes it a good bridge between:

- mathematical modeling,
- software performance work,
- browser systems work,
- inverse recovery experiments.

## What was built

The repo now contains:

- a native Rust scalar solver,
- Python and NumPy references for correctness checks,
- WebAssembly exports for browser and Node.js use,
- a separate WASM SIMD build,
- a browser render benchmark page,
- a browser inverse-recovery page,
- a Web Worker path so the browser inverse loop does not block the main page,
- a paper, experiment log, and reproducibility docs.

## What was measured

The measured work falls into five groups:

1. **Correctness**
   Rust vs Python, Rust vs NumPy, Rust vs WASM, scalar vs SIMD.
2. **Forward performance**
   Native Rust, scalar JavaScript, scalar WASM, SIMD WASM.
3. **Browser rendering**
   Field-to-RGBA conversion, `ImageData`, `putImageData`, and related browser
   costs.
4. **Gradient quality and cost**
   Forward-mode AD vs central finite differences.
5. **Inverse recovery**
   Grid search, noise sensitivity, and browser-side AD-line optimization.

## Why students should care

This repo is useful for learning because it shows a full research artifact
instead of only a final plot:

- the theory,
- the implementation,
- the benchmarking,
- the browser deployment,
- the limitations,
- the paper-writing side of the work.

That is closer to real systems research than a single script or demo.
