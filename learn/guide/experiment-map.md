---
sidebar_position: 6
title: Experiment Map
---

# Experiment Map

This chapter is about evidence.

The repo does not make its case from one benchmark or one pretty pattern. It
builds the argument in layers:

1. **Can we trust the forward solver at all?**
2. **How fast is each runtime path once trust is established?**
3. **What extra browser-side costs appear after the solver itself is done?**
4. **Can the inverse side recover parameters, and what are the tradeoffs?**

That is why the experiment log is long. Each section is answering a different
question.

## How to read the experiment chapter

Each experiment page follows the same frame:

- **Question**: what the experiment is trying to answer.
- **Why this exists**: why this question matters in the larger research story.
- **Method**: what was run and what was held fixed.
- **Metric**: what number was measured.
- **What happened**: the result in plain language.
- **Tradeoff**: why this method was chosen and what it does not prove.

If a result looks modest, keep it. The point of this guide is to explain the
artifact honestly, not to edit out inconvenient measurements.

## How to read the tables

The experiment pages use a small set of repeated metrics. They mean:

- **MAE**:
  mean absolute error. Average absolute field difference. Lower is better.
- **RMSE**:
  root mean squared error. Like MAE, but larger local mistakes are penalized
  more strongly.
- **MaxErr**:
  largest absolute difference anywhere in the field.
- **ms/step**:
  average solver time per simulation step.
- **ms/frame**:
  average rendering-side time per displayed frame.
- **ms/iteration**:
  average optimizer-loop time per iteration.
- **ms/evaluation**:
  average time per forward-loss-style evaluation inside an inverse method.
- **loss**:
  final mismatch score between a simulated field and the target field. Lower is
  better.
- **evaluated**:
  number of candidate losses or equivalent solver-backed objective evaluations
  used by the method.
- **checksum**:
  a compact consistency signal over final field values. It is not a proof of
  correctness, but it is useful for catching obvious drift or skipped work.

When you read the tables, keep two distinctions clear:

1. **correctness metrics** like MAE and RMSE ask whether two implementations
   agree;
2. **performance metrics** like ms/step or ms/frame ask how much time the path
   takes.

Those are different questions and should not be mixed.

## The four experiment families

## 1. Forward correctness

This family asks:

> Does the Rust solver agree with independent reference implementations?

That includes:

- Rust vs scalar Python,
- Rust vs NumPy `float32`,
- full-field metrics across multiple grids and step counts,
- multiple parameter regimes,
- WASM full-field checks,
- scalar vs SIMD agreement.

Without this family, every later speed number is suspect.

## 2. Forward performance

This family asks:

> Once we trust the solver, how much work does each runtime path take?

That includes:

- native Rust scalar timing,
- Node.js JavaScript timing,
- Node.js scalar WASM timing,
- WASM boundary overhead,
- zero-copy field-view timing,
- grayscale render-buffer timing,
- SIMD speedup timing.

This family is where the repo separates:

- pure solver cost,
- JS/WASM boundary cost,
- data-export cost,
- render-preparation cost,
- vectorization benefit.

## 3. Browser delivery

This family asks:

> What changes when the solver leaves a benchmark loop and becomes a browser
> page?

That includes:

- manual Chrome render measurements,
- automated headless Chrome render checks,
- browser inverse page behavior,
- Web Worker execution,
- headless browser inverse timings.

This matters because "WASM works in Node" and "the browser path is usable" are
not the same claim.

## 4. Inverse recovery

This family asks:

> Can the artifact recover `F` and `k` from a final observed pattern, and what
> does each inverse method cost?

That includes:

- dense grid-search baseline,
- finite-difference gradient baseline,
- fixed-step optimizer,
- forward-mode AD gradient check,
- AD vs finite-difference overhead comparison,
- multi-regime recovery,
- noise sensitivity,
- fixed-step AD vs backtracking AD.

This family is the most nuanced one. Lower loss is useful, but it is not the
same thing as lower parameter error, and the guide calls that out explicitly.

## Why the experiment order matters

The order is not arbitrary.

- Validation comes before performance because wrong fast code is worthless.
- Solver timing comes before browser timing because rendering and UI can hide
  where the cost really lives.
- Grid search comes before AD optimization because you need a brute-force
  baseline before claiming a smarter method helps.
- Noise comes after clean recovery because you first need to know what happens
  in the ideal case.

So the chapter is not just a result dump. It is a sequence of control
questions.

## What this chapter is not doing

It is not trying to claim:

- that CPU-only is universally best,
- that WASM always beats JavaScript by a huge factor,
- that browser timing on one machine is a universal truth,
- that a two-parameter inverse study solves general inverse PDE design.

The stronger claim is narrower:

> For this artifact, each major path was measured separately enough that the
> tradeoffs are inspectable rather than hidden.

## What to trust most in this chapter

If you want the shortest possible reading of the evidence:

- **Strongest correctness evidence**:
  full-field Rust-vs-NumPy agreement across multiple grids, steps, and regimes.
- **Strongest forward-performance evidence**:
  native/JS/WASM/SIMD tables together, especially the SIMD speedup and the
  modest scalar WASM-vs-JS result.
- **Strongest browser evidence**:
  render-path breakdown plus headless browser inverse timing with `worker: yes`.
- **Strongest inverse evidence**:
  AD-vs-FD gradient agreement, AD-overhead table, and backtracking-optimizer
  improvement over fixed-step AD.
- **Most qualified evidence**:
  single-machine browser timings and the small two-parameter inverse setup.

That is the honest center of gravity of the artifact.
