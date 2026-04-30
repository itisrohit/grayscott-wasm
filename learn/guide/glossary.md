---
sidebar_position: 10
title: Glossary
---

# Glossary

## AD

Automatic differentiation. A way to compute derivatives by propagating
derivative information through a program.

## Armijo backtracking

A line-search rule that shrinks a step size until the loss decreases enough to
accept the step.

## Benchmark

A structured timing experiment, not just one quick run.

## Browser main thread

The thread that handles UI, interaction, and much of visible page work.

## Checksum

A compact summary number used here as a quick consistency signal to confirm two
execution paths did the same simulation work.

## Command-line runner

A small program used to launch one focused experiment from the terminal. In this
repo many of these live in `src/bin/`.

## Dense grid search

Trying many parameter pairs from a predefined grid instead of using gradient
information.

## Double buffering

Using separate current-state arrays and next-state arrays so a simulation step
does not accidentally read values that were already partially updated.

## Finite differences

A derivative estimate formed by rerunning a function with slightly changed input
values.

## Gray-Scott

A two-species reaction-diffusion model often used to generate pattern-forming
simulations.

## Headless Chrome

Chrome running without the usual visible browser window, useful for automation.

## Inverse problem

Recovering unknown parameters from observed outcomes. In this project that means
starting with a final Gray-Scott pattern and trying to estimate the hidden
values of `F` and `k` that likely produced it.

## Loss function

A score that says how far a guessed result is from the target result. Lower loss
means a closer match.

## Node.js

A JavaScript runtime outside the browser. In this repo it is used for several
WASM and JS benchmarks.

## Slice

A borrowed view of existing array data. In Rust, a slice like `&[f32]` lets
code read array contents without taking ownership of the whole array.

## NumPy float32

NumPy arrays using 32-bit floating-point values. This is an important reference
because the Rust solver also uses `f32`.

## Reference implementation

An independent implementation used as a comparison point to help validate the
main implementation.

## Regression test

A test that protects a known-good behavior from changing accidentally in later
code updates.

## Periodic boundary

A boundary rule where one side of the grid wraps around to the opposite side.

## SIMD

Single instruction, multiple data. One operation processes multiple values in
parallel.

## Scalar path

The ordinary one-value-at-a-time computation path, used as the baseline when
comparing against SIMD.

## Stencil

A local update pattern where each cell depends on neighboring cells.

## WASM

WebAssembly. A low-level binary format that browsers and other runtimes can
execute efficiently.

## Web Worker

A browser worker that runs JavaScript separately from the main page thread.

## Zero-copy view

A way to let one part of a program look directly at existing memory instead of
creating a full copied duplicate of the data first.
