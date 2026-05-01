---
sidebar_position: 9
title: Reproduce the Results
---

# Reproduce the Results

This page is the practical chapter. Its job is not to repeat every measured
table in the repo. Its job is to tell you:

- what to run,
- why you are running it,
- what kind of output to expect,
- where the canonical records live if you want the full artifact trail.

## Before you start

You will usually need:

- Rust and Cargo,
- Node.js,
- Python,
- `uv` for the Python environment.

If you only want the browser pages, you can skip some of the Python-side
reference tools. If you want to reproduce the validation and paper-facing
numbers, use the full setup.

## The shortest path

If you want the fastest sanity check:

```bash
uv sync
bash tools/quality.sh
```

If that passes, the main validation and build path is healthy.

## What does the quality gate actually check?

The quality gate is the repo’s first-line safety check.

It includes:

- formatting checks,
- lint checks,
- Rust tests,
- Python checks,
- reference-comparison checks.

So when `bash tools/quality.sh` passes, it does **not** mean every benchmark in
the repo was rerun. It means the main trust and code-health checks still hold.

## Choose the path you actually want

Most readers do **not** need to run everything.

Use this split:

- **I want a quick confidence check**:
  run the quality gate.
- **I want to open the browser pages**:
  build the web package and serve `www/`.
- **I want repeatable local browser timings**:
  run the headless browser scripts.
- **I want the paper-facing tables**:
  use the reproducibility checklist and experiment log.

## Reproduce browser pages

Build the browser package:

```bash
bash tools/build_wasm_web.sh
```

Serve the repo root:

```bash
python3 -m http.server 8000
```

Open these pages:

- `http://localhost:8000/www/render_bench.html`
- `http://localhost:8000/www/inverse.html`

What these pages are for:

- `render_bench.html`:
  interactive browser render-cost breakdown.
- `inverse.html`:
  browser inverse-recovery page using the worker-backed WASM optimizer.

## Reproduce forward runtime checks

Native Rust scalar:

```bash
cargo run --release --bin bench_forward -- --grids 128,256,512 --steps 500 --trials 5
```

Node.js JavaScript:

```bash
node tools/bench_forward_js.mjs --grids 128,256,512 --steps 500 --trials 5
```

Node.js scalar WASM:

```bash
bash tools/build_wasm_node.sh
node tools/bench_forward_wasm.mjs --grids 128,256,512 --steps 500 --trials 5
```

Node.js SIMD WASM:

```bash
bash tools/build_wasm_node_simd.sh
node tools/bench_forward_wasm_simd.mjs --grids 128,256,512 --steps 500 --trials 5
```

Use these when your question is:

> How fast is the solver path itself, before browser rendering enters the
> picture?

## Reproduce validation checks

Summary comparisons:

```bash
.venv/bin/python tools/compare_numpy_reference.py
.venv/bin/python tools/compare_scalar_reference.py
```

Full-field metrics:

```bash
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000
```

WASM full-field metrics:

```bash
.venv/bin/python tools/wasm_full_field_metrics.py --width 64 --height 64 --steps 100 500 1000
```

Use these when your question is:

> Does this implementation still agree with the reference paths?

## Reproduce headless browser checks

Render benchmark:

```bash
node tools/run_browser_render_bench.mjs --grid 512 --frames 300 --steps 250
```

Inverse benchmark:

```bash
node tools/run_browser_inverse_bench.mjs --grid 64 --steps 100 --iterations 8
```

Use these when your question is:

> Can I rerun the browser-side measured path without opening and clicking
> through the pages manually?

## Reproduce inverse-method checks

Grid-search baseline:

```bash
cargo run --release --bin inverse_grid -- \
  --width 64 --height 64 --steps 100 \
  --feed-min 0.058 --feed-max 0.062 --feed-count 5 \
  --kill-min 0.060 --kill-max 0.064 --kill-count 5
```

Finite-difference gradient:

```bash
cargo run --release --bin inverse_grad -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Backtracking AD optimizer:

```bash
cargo run --release --bin inverse_ad_opt -- \
  --width 64 --height 64 --steps 100 \
  --noise-levels 0.000,0.020,0.050,0.100 \
  --seeds 24301,24589,51966,48879 \
  --iterations 8 --learning-rate 0.0001 \
  --line-learning-rate 0.001 --line-shrink 0.5 \
  --line-armijo 0.0001 --line-min-step 0.00000001 \
  --line-max-backtracks 12 \
  --feed-min 0.045 --feed-max 0.070 --feed-count 51 \
  --kill-min 0.055 --kill-max 0.070 --kill-count 31
```

Use these when your question is:

> What do the inverse-recovery methods actually do on the measured benchmark
> cases?

## Reproduce paper-facing numbers

Use:

- [Artifact reproducibility checklist](https://github.com/itisrohit/grayscott-wasm/blob/main/docs/reproducibility.md)
- [Full experiment log](https://github.com/itisrohit/grayscott-wasm/blob/main/docs/experiment-log.md)

Those are the canonical artifact records. This page is the student-friendly
pointer, not the source of truth for every exact table value.

## What output should you compare?

Different commands produce different evidence.

- For validation commands, compare error metrics like `MAE`, `RMSE`, and
  `MaxErr`.
- For forward benchmarks, compare `ms/step`.
- For render benchmarks, compare `ms/frame`.
- For inverse methods, compare:
  - `loss`,
  - `evaluated`,
  - parameter error,
  - and, when relevant, `ms/evaluation`.

If you compare the wrong metric, you can talk yourself into the wrong
conclusion.

## What to do if a number changes

If you rerun a benchmark and a number moves:

1. confirm you used the same command,
2. confirm the same environment,
3. decide whether the difference is noise or a real change,
4. update the experiment log and paper only if the new number is the one you
   intend to claim.

## Common mistakes when reproducing

The most common mistakes are simple:

1. **Mixing environments**
   Running one command with one toolchain version and comparing it against an
   older logged result from another environment.
2. **Comparing different metrics**
   For example, comparing `ms/frame` against `ms/step`, or `clean loss`
   against `noisy loss`.
3. **Ignoring warmup and trials**
   One quick run is not the same thing as the recorded benchmark protocol.
4. **Treating manual and headless browser runs as identical**
   They are related, but not interchangeable.
5. **Comparing a copied-field path against a zero-copy-view path as if they
   measured the same thing**
   They answer different boundary-cost questions.
6. **Using parameter error as the only inverse metric**
   In this repo, field loss and parameter error can disagree.

If a result looks confusing, the first thing to ask is:

> Am I comparing the same command, the same setup, and the same metric?

## What this page is not trying to do

This page is not a full lab notebook.

For that, use:

- [docs/reproducibility.md](https://github.com/itisrohit/grayscott-wasm/blob/main/docs/reproducibility.md)
- [docs/experiment-log.md](https://github.com/itisrohit/grayscott-wasm/blob/main/docs/experiment-log.md)

This page is the shortest guided route through the artifact.
