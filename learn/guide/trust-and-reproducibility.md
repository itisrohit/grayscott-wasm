---
sidebar_position: 7
title: Trust and Reproducibility
---

# Trust and Reproducibility

This page explains how the repo tries to make its claims believable instead of
just fast-looking.

## Benchmarking

A benchmark is not just “run it once and see a time.”

Good benchmarking asks:

- what exactly is being timed,
- how many trials were run,
- whether the result is a median or a single measurement,
- whether setup costs were included,
- whether different environments are being mixed unfairly.

This repo takes that seriously:

- Node.js and browser timings are separated,
- scalar and SIMD are checked against each other,
- manual browser and headless browser measurements are not merged blindly.

## Deterministic Seeds and Reproducibility

When noise is added in the inverse experiments, the repo does **not** just use
"whatever random values happen to appear."

Instead, it uses explicit seeds.

Why that matters:

- the same seed gives the same noise pattern again,
- experiments can be rerun fairly,
- comparisons between methods become more defensible.

In everyday language:

> a seed is the starting value that makes pseudo-random behavior repeatable.

That is why the paper can talk about multiple noise seeds without turning the
whole experiment into guesswork.

## Checksums, Regression Tests, and Reference Checks

This repo uses several different kinds of trust checks:

- full-field comparisons against Python and NumPy,
- scalar-vs-SIMD comparisons,
- regression tests for known runs,
- simple checksum-style consistency signals.

Why so many layers?

Because one kind of check can miss what another kind catches.

- a checksum is quick but coarse,
- a full-field comparison is stronger but more expensive,
- a regression test protects against accidental future drift.

That layered style is part of what makes the artifact credible.

## Assertions and Guard Rails

The Rust code uses checks such as:

- width and height must be non-zero,
- field lengths must match the grid size,
- some parameters like epsilon must be positive.

Those checks matter because they catch invalid states early instead of letting
bad inputs silently poison results.

For research code, that is more important than it sounds. Silent mistakes are
often worse than loud crashes.

## Headless Browser Benchmarking

Headless Chrome means Chrome is running without opening the usual visible browser
window.

Why use it:

- easier automation,
- repeatable scripts,
- cleaner local benchmarking.

Why not trust it blindly:

- it is still only one browser engine,
- it may behave differently from an interactive visible browser session,
- it may not represent mobile performance.

That is why the repo keeps manual browser numbers and headless browser numbers
as related but separate evidence.
