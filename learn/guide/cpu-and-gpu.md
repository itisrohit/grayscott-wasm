---
sidebar_position: 5
title: CPU and GPU Thinking
---

import { CpuGpuVisualizer, CpuPipelineVisualizer } from "../src/components/GuideVisuals";

# CPU and GPU Thinking

## First: what is a CPU?

A CPU is the general-purpose brain of the computer.

It is good at:

- handling many different kinds of instructions,
- making decisions,
- following branches and conditionals,
- coordinating the rest of the system,
- doing medium-sized numerical work without special infrastructure.

For a beginner, a good mental model is:

> A CPU is a flexible manager that can do almost any task, even if it is not
> always the fastest possible worker for every single type of task.

## What is a GPU?

A GPU is a processor built for huge amounts of parallel work.

It is especially good when:

- the same operation is repeated on many data items,
- thousands of small work items can run in parallel,
- the workload is regular and highly structured.

Beginner mental model:

> A GPU is less like one flexible manager and more like a giant factory floor
> with many workers doing similar operations at the same time.

## A simple picture

<CpuGpuVisualizer />

## Why not just say “GPU is faster”?

Because that sentence is too vague to be useful.

A GPU can be much faster for some workloads, but:

- data transfer costs matter,
- programming complexity matters,
- browser API support matters,
- debugging gets harder,
- reproducibility gets harder,
- not every workload is large enough to justify the extra machinery.

So the better question is:

> Faster for what exact workload, under what exact constraints?

## How this project uses the CPU

This repo uses the CPU for:

- forward simulation,
- AD-based inverse optimization,
- browser-side field-to-pixel conversion,
- SIMD vectorization inside WASM.

That means the project is not doing the most extreme form of acceleration. It
is doing something more inspectable:

- plain scalar CPU work,
- then better CPU work with SIMD,
- then browser delivery of that same compute path.

## What extra thing are we trying to prove with the CPU path?

Not just that “the CPU can run it.”

The stronger systems question is:

> Can a CPU-side Rust/WASM artifact stay understandable, reproducible, and
> browser-deliverable while still doing more than a toy visual demo?

That is why this project measures:

- correctness,
- scalar performance,
- SIMD speedup,
- render overhead,
- inverse-recovery cost,
- worker-backed browser execution.

## A little math without the heavy notation

The Gray-Scott solver updates many numbers again and again.

At a very simple level, each cell is doing something like:

```text
next value = old value + spread term + reaction term
```

Then the solver repeats that over the whole grid and over many time steps.

That means the compute pattern is:

- many arithmetic operations,
- repeated many times,
- over arrays of floating-point numbers.

That is exactly why CPU layout, caching, and SIMD matter here. Even without a
GPU, a lot of performance depends on how efficiently those repeated math
operations move through memory.

## A low-level view of the CPU-side story

<CpuPipelineVisualizer />

In plain language:

- values live in arrays,
- the CPU reads nearby values,
- computes the next step,
- writes the results back,
- repeats that many times,
- and then the browser page can render or analyze the output.

## What SIMD changes

SIMD is still CPU compute.

It does **not** turn the CPU into a GPU.

It simply lets one instruction handle several nearby values together. In this
project that is used inside the WASM SIMD build to speed up the forward kernel.

## Why this matters for the browser

Browsers already have plenty of moving parts:

- JavaScript,
- WASM,
- canvas,
- workers,
- security sandboxing,
- different browser engines.

Adding GPU compute would introduce even more:

- WebGPU or shader APIs,
- more backend-specific debugging,
- more hardware variance,
- more compatibility questions.

So a CPU-first browser artifact is a deliberate research scope, not a fallback
born from ignorance.

## When a GPU version would make sense

A GPU version would make more sense if the project wanted to:

- push much larger grids,
- recover many more parameters,
- run many simulations in parallel,
- do real-time heavy interactive exploration,
- compare against established GPU-first differentiable solvers.

That would be a good future project. It is simply not the same project as this
one.
