# Docs Map

Use this directory as the navigation layer for the research artifact.

## Read These In Order

1. [../README.md](../README.md)
   Setup, common commands, and the fastest path to running the artifact.
2. [experiment-log.md](experiment-log.md)
   Measured results, validation commands, benchmark tables, and observed output.
3. [plan.md](plan.md)
   Long-form research framing, current artifact status, and remaining optional
   work.
4. [reproducibility.md](reproducibility.md)
   Paper-oriented reproduction path for the current benchmark and browser tables.
5. [manualcheck-browser-render.md](manualcheck-browser-render.md)
   Manual and headless procedures for the browser render benchmark.
6. [research-directions.md](research-directions.md)
   Literature-backed ideas for stronger future work.

## What To Open For Specific Tasks

- Reproduce measured results:
  [reproducibility.md](reproducibility.md)
- Inspect the full chronological log:
  [experiment-log.md](experiment-log.md)
- Understand the paper framing and current completion status:
  [plan.md](plan.md)
- Run the browser render page carefully:
  [manualcheck-browser-render.md](manualcheck-browser-render.md)
- Decide what future work is actually worth doing:
  [research-directions.md](research-directions.md)
- Read or edit the paper:
  [../paper/main.tex](../paper/main.tex)
- View the latest compiled paper PDF:
  [../paper/grayscott_wasm_IEEE_Journal_Paper.pdf](../paper/grayscott_wasm_IEEE_Journal_Paper.pdf)

## Current State

- Core implementation is complete for the present Rust/WASM paper scope.
- SIMD results are implemented and measured.
- Browser inverse recovery runs through a module Web Worker.
- Remaining work is mostly paper/submission polish or optional cross-browser
  measurements.
