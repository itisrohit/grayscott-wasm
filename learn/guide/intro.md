---
sidebar_position: 1
title: Welcome
---

# Welcome

This site explains the `grayscott-wasm` research artifact for readers who may
be completely new to:

- reaction-diffusion systems,
- partial differential equations,
- Rust,
- WebAssembly,
- browser benchmarking,
- automatic differentiation,
- inverse problems.

## What you will get from this guide

By the end, you should understand:

- what the Gray-Scott model is trying to simulate,
- what the repo actually built,
- why there are so many benchmarks and validation scripts,
- what the measured numbers mean,
- what ideas from Rust and computer science make the implementation work,
- where the project is strong and where it is limited.

## How to read this

The guide is split into chapters.

- **Big Picture** explains the research question in plain language.
- **Computing Ideas** explains the software and systems concepts, including how
  browsers load WASM and how CPU and GPU thinking differ.
- **Common Questions** answers things like "Why CPU instead of GPU?" and "How
  does WASM use memory inside a browser?"
- **Experiments** explains the benchmark and inverse-recovery results.
- **Use It Yourself** explains how to rerun the artifact and decode the terms.

## The short version

This project asks a focused question:

> Can a browser-deliverable Rust/WebAssembly implementation do more than show a
> Gray-Scott pattern? Can it also support measured, reproducible inverse
> recovery experiments?

The answer from the current artifact is **yes, for a small two-parameter
problem**, with important limits:

- the solver is validated carefully,
- the WASM SIMD path is much faster than scalar WASM in Node.js,
- the browser inverse loop works and now runs off the main thread in a Web
  Worker,
- but the inverse problem is still small, CPU-only, and measured on one machine.
