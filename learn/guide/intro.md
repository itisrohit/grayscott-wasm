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
  browsers load WASM, how CPU and GPU thinking differ, how the solver is
  organized, and how the repo keeps its results trustworthy.
- **Math and Algorithms** explains the actual Gray-Scott update rules, the
  discrete solver, the loss function, and the inverse-search methods in the
  same order they appear in the code.
- **Common Questions** answers things like "Why CPU instead of GPU?" and "How
  does WASM use memory inside a browser?"
- **Experiments** explains the benchmark and inverse-recovery results.
- **Use It Yourself** explains how to rerun the artifact and decode the terms.

## Recommended reading order

If you are completely new, do not read the guide like a paper from front to
back without a plan.

Use one of these routes:

### Route A: absolute beginner

1. **Big Picture**
2. **Gray-Scott Basics**
3. **Rust and WebAssembly**
4. **CPU and GPU Thinking**
5. **Common Questions**
6. **Experiments**
7. **Use It Yourself**

This route is best if words like PDE, WASM, SIMD, and inverse problem still
feel unfamiliar.

### Route B: technical reader

1. **Big Picture**
2. **Math and Algorithms**
3. **Experiments**
4. **Computing Ideas**
5. **Use It Yourself**

This route is better if you already know the vocabulary and want the technical
argument first.

### Route C: I only care about rerunning it

1. **Use It Yourself**
2. **Experiments**
3. **Glossary**

This route is for readers who mainly want to reproduce commands and interpret
the resulting numbers.

## What each chapter is trying to do

Each chapter has a different job:

- **Chapter 1** gives the research question and the physical intuition.
- **Chapter 2** explains the software and systems ideas the repo relies on.
- **Chapter 3** explains the math and algorithms in the same order they matter
  in the code.
- **Chapter 4** explains the evidence, measured tables, and tradeoffs.
- **Chapter 5** tells you how to rerun the artifact and decode the repeated
  terms.

That division is deliberate. It keeps the guide from mixing:

- explanation,
- derivation,
- benchmark interpretation,
- and practical reproduction steps

all on the same page.

## What to do when a page feels too technical

Use the guide sideways, not only downward.

If a page becomes too technical:

- jump to **Glossary** for the terms,
- jump to **Common Questions** for simpler framing,
- return to **Math and Algorithms** only after the basic picture is clear.

This is a teaching guide, not an exam. You are allowed to move between
chapters.

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
