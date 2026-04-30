---
sidebar_position: 6
title: Code Organization and Interfaces
---

# Code Organization and Interfaces

This page explains how the repo is organized so that the simulation, inverse
logic, browser layer, and experiment runners do not all collapse into one file.

## Configuration Structs

Rust code in this repo uses many small `struct`s to keep related values
together.

Examples include:

- `GrayScottParams`
- `InverseTarget`
- `GridSearchConfig`
- `GradientDescentConfig`
- `BacktrackingConfig`

Beginner translation:

> a struct is a named bundle of related data.

That is better than passing long loose argument lists everywhere, because the
meaning stays clearer and the code is easier to validate.

Another advantage is that configs can be reused across:

- native experiments,
- browser-facing wrappers,
- command-line tools,
- tests.

## Modules and Separation of Responsibility

The repo is split by responsibility:

- `solver.rs` handles forward simulation,
- `inverse.rs` handles inverse-recovery logic,
- `wasm.rs` exposes the browser-facing WASM API,
- `src/bin/` contains command-line experiment entry points.

That separation matters because research code gets messy quickly if every idea
is mixed into one file.

So one lesson from this repo is not just numerical. It is organizational:

> separate simulation, optimization, browser interface, and experiment runners
> so each part stays inspectable.

## Command-Line Experiment Runners

The files in `src/bin/` are small command-line programs built on top of the
core library.

They exist so the project can run focused experiments like:

- forward benchmarks,
- inverse overhead checks,
- noise sweeps,
- regime comparisons.

That is an important research-software pattern:

> keep the reusable core logic in the library, and put one-off experiment entry
> points in separate small runner programs.

## Browser-Facing Wrapper Types

The browser does not talk directly to the whole Rust crate in the same way that
another Rust file would.

Instead, the repo exposes a browser-facing wrapper such as `WasmGrayScott`.

Why do that?

- it gives JavaScript a cleaner API,
- it hides internal Rust details,
- it lets the WASM layer decide what should be exported and what should stay
  internal.

That is why the browser side sees a curated interface instead of every solver
detail.

## Typed Arrays

On the browser side, data often appears as:

- `Float32Array`
- `Uint8ClampedArray`

These are JavaScript array types designed for raw numeric data.

Why they matter here:

- `Float32Array` matches solver-style floating-point field data,
- `Uint8ClampedArray` matches RGBA pixel buffers for browser rendering.

So when the guide talks about "views into WASM memory," these typed arrays are
the practical mechanism doing that work.

## Why `unsafe` Appears in the WASM Layer

Most of the Rust code is ordinary safe Rust. One notable exception is the
browser-facing typed-array view creation.

Why?

Because exposing a raw view into WASM memory is a low-level operation. The code
is promising that the memory layout and lifetime assumptions are being handled
carefully enough for JavaScript to look directly at that buffer.

Beginner translation:

> `unsafe` does not mean "wrong." It means "the compiler needs the programmer
> to take extra responsibility here."

In this repo, that responsibility is used narrowly to support zero-copy browser
views.

## JSON As a Boundary Format

One browser-facing inverse entry point returns a JSON string rather than a very
complicated nested Rust object.

Why use JSON there?

- JavaScript understands it naturally,
- browser pages can display or log it easily,
- it simplifies a complicated cross-language boundary.

This is another pragmatic engineering choice:

> use rich low-level views for heavy numeric arrays, and use a simple text
> format for structured status/results when that is easier to handle.
