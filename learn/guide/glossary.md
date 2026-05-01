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

## Browser render path

The part of the browser-side workflow that turns solver fields into visible
pixels on screen.

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

## Environment

The combination of runtime, operating system, compiler, browser version, and
tool versions used for an experiment.

## Double buffering

Using separate current-state arrays and next-state arrays so a simulation step
does not accidentally read values that were already partially updated.

## Finite differences

A derivative estimate formed by rerunning a function with slightly changed input
values.

## Gray-Scott

A two-species reaction-diffusion model often used to generate pattern-forming
simulations.

## Headless browser check

A scripted browser run used for repeatable local measurement without manual page
interaction.

## Headless Chrome

Chrome running without the usual visible browser window, useful for automation.

## ImageData

A browser image object that stores pixel data for canvas drawing.

## Inverse problem

Recovering unknown parameters from observed outcomes. In this project that means
starting with a final Gray-Scott pattern and trying to estimate the hidden
values of `F` and `k` that likely produced it.

## Loss

Short for loss function. The mismatch score the inverse methods try to reduce.

## Loss function

A score that says how far a guessed result is from the target result. Lower loss
means a closer match.

## MAE

Mean absolute error. Average absolute difference between two fields.

## MaxErr

Maximum absolute error. The single largest pointwise difference between two
fields.

## Metric

A measured quantity used to compare implementations or experiments.

## ms/evaluation

Average milliseconds per solver-backed objective evaluation.

## ms/frame

Average milliseconds spent per displayed frame in a render benchmark.

## ms/iteration

Average milliseconds spent per optimizer iteration.

## ms/step

Average milliseconds spent per forward simulation step.

## Node.js

A JavaScript runtime outside the browser. In this repo it is used for several
WASM and JS benchmarks.

## Slice

A borrowed view of existing array data. In Rust, a slice like `&[f32]` lets
code read array contents without taking ownership of the whole array.

## NumPy float32

NumPy arrays using 32-bit floating-point values. This is an important reference
because the Rust solver also uses `f32`.

## OffscreenCanvas

A browser canvas API that can be used away from the visible page canvas and is
often helpful for worker-friendly rendering paths.

## Parameter error

Difference between a recovered parameter value and the generating ground-truth
parameter value.

## Protocol

The exact experimental setup: commands, grid sizes, step counts, warmup, trials,
and related settings.

## putImageData

A browser canvas operation that writes raw pixel data into a 2D canvas.

## Reference implementation

An independent implementation used as a comparison point to help validate the
main implementation.

## Regression test

A test that protects a known-good behavior from changing accidentally in later
code updates.

## RMSE

Root mean squared error. Like MAE, but larger local differences are weighted
more strongly.

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

## Warmup

Untimed work done before a benchmark so the measured trials better reflect the
steady path instead of first-run effects.

## Web Worker

A browser worker that runs JavaScript separately from the main page thread.

## Worker-backed inverse path

The browser inverse page’s execution path where the heavy optimizer runs inside
a Web Worker instead of the main UI thread.

## Zero-copy view

A way to let one part of a program look directly at existing memory instead of
creating a full copied duplicate of the data first.
