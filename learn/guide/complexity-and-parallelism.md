---
sidebar_position: 11
title: Complexity and Parallelism
---

# Complexity and Parallelism

This page explains the part that is easy to skip but important to understand:

- how much work the repo’s algorithms actually do,
- why the CPU version works as a legitimate baseline,
- what SIMD changes mathematically,
- and what a GPU version would probably look like if this repo had taken that
  path.

## Why a cost model belongs in the math chapter

The experiments are not only about “does it work?” They are also about:

- how much work each method does,
- how that work scales with grid size,
- how memory pressure changes the practical result,
- why one algorithm becomes preferable to another.

That means complexity is part of the algorithm story, not just implementation
detail.

## The forward solver cost model

Let:

- $W$ = grid width
- $H$ = grid height
- $N = WH$ = number of cells
- $S$ = number of time steps

### Cost of one cell update

One scalar cell update does a constant amount of work:

- read the center value,
- read four neighbors,
- compute two Laplacians,
- compute the reaction term,
- write two outputs.

So one cell update is:

```math
O(1)
```

### Cost of one full time step

A full time step updates every cell once, so:

```math
T_{\text{step}}(W,H) = O(WH) = O(N)
```

### Cost of one whole forward simulation

Running the solver for $S$ steps gives:

```math
T_{\text{forward}}(W,H,S) = O(SWH) = O(SN)
```

That is the main reason the experiments become visibly slower as:

- the grid gets larger,
- or the step count gets larger.

## The forward memory model

The primal solver stores four float arrays:

- current $u$
- current $v$
- next $u$
- next $v$

Each `f32` is 4 bytes, so the raw field-storage cost is:

```math
M_{\text{primal}} = 4 \text{ arrays} \times 4 \text{ bytes} \times N = 16N \text{ bytes}
```

That ignores object overhead and other runtime details, but it gives the right
core model.

## The forward-mode AD memory model

The `Dual2` representation stores three `f32` values per logical number:

- value
- derivative with respect to $F$
- derivative with respect to $k$

So each dual cell entry is:

```math
3 \times 4 = 12 \text{ bytes}
```

The AD path still stores four arrays:

- current $u$
- current $v$
- next $u$
- next $v$

So the raw dual-field storage cost is:

```math
M_{\text{AD}} = 4 \times 12 \times N = 48N \text{ bytes}
```

That is a 3x increase over the primal field storage:

```math
\frac{M_{\text{AD}}}{M_{\text{primal}}} = \frac{48N}{16N} = 3
```

This is the clean mathematical reason the AD path can show more cache pressure
than the primal path even when the algorithmic structure is similar.

## Why the CPU version parallelizes at all

Within a single time step, every next-state cell depends only on the current
buffers, not on other next-state writes.

That means:

- cell $(i,j)$ can be updated independently of cell $(p,q)$,
- as long as both read only from the current state and write to separate next
  locations.

Double buffering is what makes this possible.

Without double buffering, the update graph would become order-dependent.

So the algorithm has a natural per-step parallel structure:

```math
\text{all cells in one step are data-parallel}
```

but the time steps themselves remain sequential:

```math
n \rightarrow n+1 \rightarrow n+2 \rightarrow \cdots
```

That is the basic dependency pattern of this solver.

## What SIMD changes mathematically

SIMD does not change the asymptotic complexity.

The forward solver remains:

```math
O(SN)
```

But SIMD changes the constant factor by doing several same-shaped arithmetic
operations in one instruction group.

In this repo’s WASM SIMD path, the interior loop works on 4 lanes at a time.
So the idealized mental model is:

```math
\text{scalar interior work} \approx 4 \times \text{SIMD-lane groups}
```

In the real world, the speedup is not exactly 4x because:

- boundaries still use the scalar path,
- loads and stores still cost time,
- memory layout and cache behavior matter,
- the engine and compiler affect instruction quality.

So SIMD improves the constant factor, not the algorithmic order.

## Cost of the inverse methods

The inverse problem repeatedly calls the forward solver, so its cost is easiest
to understand in units of “how many forward-like evaluations happen?”

### Grid search

Let:

- $C_F$ = number of feed candidates
- $C_k$ = number of kill candidates

Then grid search does:

```math
C_F C_k
```

full candidate evaluations.

Its total cost is:

```math
T_{\text{grid}} = O(C_F C_k S N)
```

### Finite-difference gradient

For $p$ parameters, central finite differences need:

```math
2p + 1
```

loss evaluations per gradient.

In this repo, $p=2$, so:

```math
2p+1 = 5
```

That is why the finite-difference gradient path is much more expensive than a
single primal loss evaluation.

### Forward-mode AD gradient

Forward-mode AD carries tangent information through one augmented solve. Its
asymptotic cost still scales with $S N$, but with a larger constant factor:

```math
T_{\text{AD-grad}} = O(SN)
```

with more arithmetic and more memory traffic than the primal path.

That is exactly why the repo compares:

- one primal loss,
- one AD gradient,
- one finite-difference gradient.

### Backtracking line search

If an iteration needs:

- one AD gradient evaluation,
- plus $b$ candidate loss checks during step shrinkage,

then one backtracking iteration is roughly:

```math
T_{\text{line-search iteration}} \approx T_{\text{AD-grad}} + b \, T_{\text{loss}}
```

That is why iteration count and evaluation count are not the same thing.

## Why the CPU-first path is still mathematically interesting

A beginner mistake is to think:

> if the GPU is faster, then studying the CPU path is not mathematically
> interesting.

That is wrong.

The CPU-first path lets the repo isolate:

- the numerical method,
- the data layout,
- the differentiation strategy,
- the search policy,
- the browser delivery mechanics.

In other words, CPU-first is not “anti-GPU.” It is a way to keep the algorithm
story inspectable.

## What a GPU version would probably look like

If this solver were mapped to a compute-style GPU pipeline, the usual design
would be:

1. store current and next fields in GPU buffers or textures,
2. launch many parallel threads,
3. give each thread responsibility for one cell,
4. have each thread read neighbor values,
5. compute the update,
6. write the result to the next buffer,
7. swap the two buffers and repeat for the next time step.

That means the GPU mental model is:

```math
\text{one thread} \leftrightarrow \text{one cell update}
```

or sometimes:

```math
\text{one workgroup} \leftrightarrow \text{one tile of the grid}
```

## What would change in a browser GPU path

In browser terms, a compute-style GPU implementation would likely use WebGPU
compute pipelines.

Conceptually:

- a compute shader would hold the cell-update rule,
- workgroups would cover chunks of the grid,
- storage buffers would hold the fields,
- JavaScript would dispatch the workgroup grid each step.

The math would be the same stencil update. The execution model would change.

## Why this repo did not take the GPU path

A GPU version could absolutely be valuable, but it would change the project in
several ways.

### What would likely improve

- forward throughput would probably increase,
- large grids would become more attractive,
- browser rendering and compute could live closer to the same hardware path.

### What would get more complicated

- browser compatibility,
- deployment requirements,
- buffer binding and shader setup,
- correctness debugging,
- GPU/CPU result comparison,
- browser-specific performance interpretation.

So the repo chose the CPU-first path because it keeps the scientific and
algorithmic claims easier to defend.

## The remaining algorithmic gap this page closes

Without this page, Chapter 3 explains:

- what the solver computes,
- and how the inverse methods work.

But it does not fully explain:

- how work scales,
- where the measured overhead comes from,
- why AD has a memory penalty,
- how SIMD changes constants but not order,
- what a GPU mapping would look like if it existed.

That missing cost-model layer is now covered here.
