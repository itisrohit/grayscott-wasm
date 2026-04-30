---
sidebar_position: 8
title: Math and Algorithms
---

# Math and Algorithms

This chapter sits between the broad ideas and the measured experiments.

Its job is simple:

- explain the actual mathematical model,
- explain how that model becomes code,
- explain how the inverse search works,
- explain the differentiation and optimization rules used in the inverse code,
- explain why this repo chose these methods and not several obvious
  alternatives.

If you want the shortest possible summary, it is this:

1. the Gray-Scott model gives local update rules for two fields, `u` and `v`;
2. `src/solver.rs` turns those rules into a repeated grid update;
3. `src/inverse.rs` turns “match this final pattern” into a search problem;
4. the repo compares several search strategies instead of trusting one
   optimizer blindly;
5. the core math is reused across scalar, SIMD, native, and browser paths;
6. `src/wasm.rs` exposes the same core logic to the browser.

## The full puzzle in one view

The whole project is built from four layers.

### 1. Continuous model

At the science level, the Gray-Scott system says:

- `u` diffuses and reacts,
- `v` diffuses and reacts,
- feed adds fresh `u`,
- kill removes `v`.

In paper language, that is a reaction-diffusion PDE model.

In beginner language, it means:

> every point changes because nearby points influence it and because local
> chemistry changes it.

### 2. Discrete simulation

A browser or CPU cannot solve “continuous space and continuous time” directly.
So the repo makes three practical replacements:

- space becomes a rectangular grid,
- time becomes repeated finite steps,
- derivatives become neighbor-based arithmetic.

That is the forward solver.

### 3. Scoring a candidate answer

Once the repo has a forward solver, it can ask:

> if I try one candidate `F, k` pair, how close does the final pattern get to
> the target pattern?

That question becomes a **loss function**. Lower loss means a better match.

### 4. Searching for good parameters

After that, the inverse problem becomes a search problem:

- grid search tries many pairs,
- finite differences estimate slope by rerunning the solver,
- forward-mode AD carries slope information through the solver directly,
- gradient descent and backtracking use that slope to move toward lower loss.

### 5. Controlling cost and credibility

The repo does not only ask:

> did some method eventually get a decent answer?

It also asks:

- how many full solver evaluations did that method need,
- how sensitive is it to noise,
- how much of the improvement comes from better math versus better heuristics,
- whether the browser path is still running the same core algorithm.

## Where each part lives in the repo

- `src/solver.rs`
  forward Gray-Scott simulation, scalar update, SIMD update, periodic
  boundaries, double buffering
- `src/inverse.rs`
  target generation, noise injection, loss, finite differences, forward-mode
  AD, optimization, grid search, deterministic pseudorandom noise
- `src/wasm.rs`
  browser-facing wrapper, typed-array views, JSON browser inverse entry point
- `src/bin/*`
  command-line runners for validation and benchmarks

## Why this chapter is split into three follow-up pages

There are really three different stories here:

- the **repo map story**:
  which structs, helper functions, and interfaces carry the math through the
  codebase;
- the **forward math story**:
  what one Gray-Scott update means and how the solver computes it;
- the **inverse math story**:
  how the repo compares guessed parameters to a target, differentiates the
  loss, and searches intelligently.

Those are the next three pages:

- [Repo Algorithm Map](./algorithm-map)
- [Forward Solver Math](./forward-solver-math)
- [Inverse Math and Search](./inverse-math-and-search)

## What kind of math you should expect here

This chapter now uses real formulas. But the formulas are always tied back to
three practical questions:

1. what physical or numerical idea is this formula expressing,
2. where does that formula appear in the repo,
3. why was this formula chosen over obvious alternatives.

So the chapter is not “math for decoration.” It is math used to explain the
actual implementation and experiments.

## What this repo deliberately did not try to do

This matters, because beginners often assume a stronger claim than the code
actually makes.

The repo does **not** try to:

- solve the PDE in the most accurate scientific way possible,
- infer a huge parameter vector,
- infer the entire initial condition,
- use GPU kernels,
- use adjoint or reverse-mode differentiation,
- optimize for giant production-scale inverse design.

Instead it tries to answer a narrower question cleanly:

> Can a browser-deliverable Rust/WASM artifact support a trustworthy forward
> solver, measured browser rendering, and a small but real inverse-recovery
> loop?

That narrower question is exactly why the algorithms in this repo are small
enough to inspect and defend.
