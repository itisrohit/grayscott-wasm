---
sidebar_position: 4
title: Rust and WebAssembly
---

# Rust and WebAssembly

## Why Rust?

Rust gives this project three useful things:

- predictable performance,
- explicit memory control,
- good support for native binaries and WebAssembly builds from the same codebase.

That makes it a strong fit for a solver that must:

- run locally for validation,
- compile to WASM for browsers,
- stay readable enough for research reproduction.

## What is WebAssembly?

WebAssembly, or WASM, is a binary format that browsers and other runtimes can
execute efficiently.

For this project, that means:

- the solver can run in the browser,
- the same core numerical logic can be reused,
- JavaScript becomes the UI and orchestration layer rather than the only compute
  layer.

## Why not just use JavaScript?

You could. In fact, this repo includes a scalar JS baseline.

But Rust/WASM is useful when you want:

- more control over memory layout,
- a clearer path to native and browser reuse,
- easier integration of low-level optimizations such as SIMD,
- a systems-style artifact rather than only a browser demo.

## What is the JS/WASM boundary?

The browser page is still JavaScript.

WASM does not replace the whole app. Instead:

- JavaScript handles the page, controls, and browser APIs,
- WASM handles heavy numerical work,
- values cross the boundary between them.

That boundary matters because copying large arrays back and forth can be slow.
This repo explicitly measures and designs around that issue.

## What is zero-copy access here?

The project exposes field views from WASM memory to JavaScript using typed
arrays. That avoids copying a whole field every time the page wants to read it.

This is one of the most important systems ideas in the repo:

- copying is simple but expensive,
- views are faster but need more care.

## Why is the browser inverse loop in a Web Worker now?

Browsers have a main thread that handles page interaction and rendering.

If you run a heavy inverse optimization directly on that thread:

- the page can freeze,
- buttons and scrolling feel blocked,
- the site feels less credible as a real browser artifact.

A Web Worker moves that heavy computation off the main thread.

In this repo:

- `www/inverse.js` runs the UI,
- `www/inverse_worker.js` runs the inverse optimizer,
- the worker returns JSON results back to the page.

That is a classic browser systems pattern: keep the UI responsive by moving
compute elsewhere.
