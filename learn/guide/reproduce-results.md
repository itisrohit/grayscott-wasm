---
sidebar_position: 9
title: Reproduce the Results
---

# Reproduce the Results

## The shortest path

If you want the fastest sanity check:

```bash
uv sync
bash tools/quality.sh
```

If that passes, the main validation and build path is healthy.

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

## Reproduce headless browser checks

Render benchmark:

```bash
node tools/run_browser_render_bench.mjs --grid 512 --frames 300 --steps 250
```

Inverse benchmark:

```bash
node tools/run_browser_inverse_bench.mjs --grid 64 --steps 100 --iterations 8
```

## Reproduce paper-facing numbers

Use:

- [Artifact reproducibility checklist](https://github.com/itisrohit/grayscott-wasm/blob/main/docs/reproducibility.md)
- [Full experiment log](https://github.com/itisrohit/grayscott-wasm/blob/main/docs/experiment-log.md)

Those are the canonical artifact records. This page is the student-friendly
pointer, not the source of truth for every exact table value.

## What to do if a number changes

If you rerun a benchmark and a number moves:

1. confirm you used the same command,
2. confirm the same environment,
3. decide whether the difference is noise or a real change,
4. update the experiment log and paper only if the new number is the one you
   intend to claim.
