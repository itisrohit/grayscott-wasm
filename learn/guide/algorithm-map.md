---
sidebar_position: 9
title: Repo Algorithm Map
---

# Repo Algorithm Map

This page is the bridge between the math story and the source code.

The previous page explained the big algorithmic puzzle. This page explains how
the repo breaks that puzzle into concrete data structures and functions.

## The main objects in the repo

### `GrayScottParams`

This is the parameter bundle for one forward run.

It stores:

- `feed`
- `kill`
- `diff_u`
- `diff_v`
- `dt`

Why bundle them into one struct instead of passing five loose numbers around?

- fewer call-site mistakes,
- easier default configuration,
- easier scalar and SIMD reuse,
- cleaner API boundaries.

### `GrayScott`

This is the forward solver state.

It stores:

- `width`
- `height`
- current `u`
- current `v`
- next `u`
- next `v`

That means one `GrayScott` value is not “just parameters.” It is the full live
state of one simulation.

### `InverseTarget`

This is the inverse-problem definition.

It stores:

- grid width and height,
- number of steps,
- seed radius,
- the true forward parameters.

Why store this as one object?

Because it defines the exact target-generation recipe. That makes the inverse
helpers easier to call consistently.

### Search and optimization config structs

The inverse code uses several config objects:

- `GridSearchConfig`
- `GradientDescentConfig`
- `BacktrackingConfig`

These encode the search rules:

- what parameter region is allowed,
- how many candidates exist,
- what step size to try,
- how backtracking shrinks it,
- how many iterations are allowed.

This is important because the inverse logic is not just “one formula.” It is a
mathematical model plus a search policy.

## The forward path in code order

The forward path is built like this:

1. create `GrayScott::new(width, height)`
2. seed the center square with `seed_square`
3. choose `GrayScottParams`
4. call `step`, `run`, or `run_simd`
5. read `u()` and `v()`

That is the base path used by:

- correctness tests,
- native benchmarks,
- WASM benchmarks,
- browser rendering,
- target generation,
- grid-search candidates,
- gradient-based inverse methods.

This shared path is one of the strongest design decisions in the repo. The same
forward logic is reused instead of maintaining separate “demo” and “research”
solvers.

## The inverse path in code order

The inverse path is built like this:

1. define an `InverseTarget`
2. generate the target fields with `generate_target`
3. optionally perturb them with `add_uniform_noise`
4. define a loss rule with `field_mse` and `loss_for_params`
5. choose a search method:
   - `grid_search`
   - `finite_difference_gradient`
   - `forward_gradient`
   - `gradient_descent`
   - `forward_gradient_descent`
   - `forward_gradient_descent_backtracking`

That is the whole inverse-research pipeline.

## Why helper functions matter mathematically

Some helper functions look “small” in code but are conceptually important.

### `seeded_sim`

This helper makes a new forward simulation and applies the standard seed.

Why is that important?

Because the inverse experiments depend on a fixed initial condition. Reusing the
same helper avoids accidental drift in setup.

### `linspace`

This creates evenly spaced parameter samples for grid search.

Mathematically, it defines the candidate lattice. That means it affects:

- search resolution,
- whether the exact target lies on the grid,
- how expensive brute force becomes.

### `field_mse`

This is the actual score the search methods optimize.

If this function changed, the meaning of “best parameter” would change too.

### `loss_for_params`

This function is the key forward-to-inverse bridge.

It means:

> take one guessed parameter pair, run the full solver, and return one scalar
> score.

That scalar score is what makes optimization possible.

## Why “evaluated” is tracked

Several inverse result structs track how many evaluations were used.

That is not cosmetic metadata. It is part of the algorithm story.

Examples:

- finite differences evaluate five losses per gradient,
- forward-mode AD evaluates one loss-and-gradient pass,
- backtracking may spend extra evaluations trying candidate step sizes.

So “evaluated” measures real algorithmic cost, not just elapsed time.

That is why the paper and docs discuss evaluation count as a first-class
comparison.

## Where determinism enters the design

The repo tries to keep experiments reproducible.

That shows up in several places:

- fixed initial seeding,
- bounded parameter ranges,
- deterministic noise generation,
- explicit test assertions,
- checksums and comparisons.

The deterministic noise path uses `SplitMix64`, a small pseudorandom generator.

Why not just call a larger random library?

- more dependency weight,
- less local transparency,
- unnecessary for a tiny bounded-noise helper.

The chosen approach is minimal and inspectable.

## How the browser path fits without changing the math

The browser-facing layer in `src/wasm.rs` does not replace the solver or the
inverse code. It wraps them.

Forward browser path:

1. create `WasmGrayScott`
2. set parameters
3. seed
4. run scalar or SIMD path
5. expose values through pointers or typed-array views

Inverse browser path:

1. receive browser arguments,
2. generate target,
3. optionally add noise,
4. run AD backtracking recovery,
5. serialize result history to JSON.

So the browser layer is best understood as:

> interface code around the same mathematical core.

## Why the repo uses both vectors and JSON

This repo crosses several boundaries:

- Rust internal code,
- Rust-to-WASM boundary,
- JS UI code,
- browser-worker messages.

That is why it uses different data shapes for different jobs:

- `Vec<f32>` for dense numeric fields,
- typed-array views for fast browser access,
- JSON strings for structured browser inverse summaries.

That mixture is not inconsistency. It is boundary-aware engineering.

## Why this page matters before reading experiments

The experiment pages are much easier to interpret if you know:

- what exactly counts as one evaluation,
- what exactly gets rerun,
- which parts are the fixed model,
- which parts are search policy choices,
- which parts are browser interface code rather than mathematical changes.

That separation is one of the main reasons this repo is readable at all.
