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

## Dense grid search

Trying many parameter pairs from a predefined grid instead of using gradient
information.

## Finite differences

A derivative estimate formed by rerunning a function with slightly changed input
values.

## Gray-Scott

A two-species reaction-diffusion model often used to generate pattern-forming
simulations.

## Headless Chrome

Chrome running without the usual visible browser window, useful for automation.

## Inverse problem

Recovering unknown parameters from observed outcomes.

## Node.js

A JavaScript runtime outside the browser. In this repo it is used for several
WASM and JS benchmarks.

## NumPy float32

NumPy arrays using 32-bit floating-point values. This is an important reference
because the Rust solver also uses `f32`.

## Periodic boundary

A boundary rule where one side of the grid wraps around to the opposite side.

## SIMD

Single instruction, multiple data. One operation processes multiple values in
parallel.

## Stencil

A local update pattern where each cell depends on neighboring cells.

## WASM

WebAssembly. A low-level binary format that browsers and other runtimes can
execute efficiently.

## Web Worker

A browser worker that runs JavaScript separately from the main page thread.
