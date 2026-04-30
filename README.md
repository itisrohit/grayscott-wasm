# grayscott-wasm

Rust/WebAssembly research artifact for a differentiable Gray-Scott
reaction-diffusion solver, browser render benchmarks, and small-parameter
inverse recovery.

## What This Repo Contains

- Native Rust scalar solver.
- Python and NumPy references for correctness checks.
- Node.js WASM and WASM SIMD benchmarks.
- Browser render benchmark page.
- Browser inverse recovery page running the optimizer in a module Web Worker.
- Experiment logs, paper source, and the latest compiled PDF.

Latest paper PDF:

- [grayscott_wasm_IEEE_Journal_Paper.pdf](paper/grayscott_wasm_IEEE_Journal_Paper.pdf)

## Start Here

Set up Python tooling:

```bash
uv sync
```

Install local git hooks:

```bash
bash tools/install-hooks.sh
```

Run the full local quality gate:

```bash
bash tools/quality.sh
```

That is the best first check if you want to confirm the repo is healthy before
running individual experiments.

## First Navigation Path

If you are new to this repo, read in this order:

1. This file for setup and common commands.
2. [docs/README.md](docs/README.md) for the documentation map.
3. [docs/experiment-log.md](docs/experiment-log.md) for measured results.
4. [paper/main.tex](paper/main.tex) for the paper source.

## Repository Map

- `src/`
  Native solver, AD/inverse logic, and WASM exports.
- `src/bin/`
  Reproducible CLI experiments and benchmarks.
- `reference/`
  Python and NumPy validation references.
- `tools/`
  Build scripts, quality checks, and browser/headless benchmark runners.
- `www/`
  Browser pages:
  `render_bench.html`, `inverse.html`, and `inverse_worker.js`.
- `docs/`
  Plan, experiment log, manual browser-check notes, research directions.
- `paper/`
  IEEE paper source, bibliography, figures, and compiled PDF.

## Most Common Tasks

### Validate The Solver

```bash
cargo test
.venv/bin/python tools/compare_scalar_reference.py
.venv/bin/python tools/compare_numpy_reference.py
.venv/bin/python tools/full_field_metrics.py --width 64 --height 64 --steps 100 500 1000
```

### Build WASM

```bash
bash tools/build_wasm_node.sh
bash tools/build_wasm_web.sh
```

### Check Node.js WASM

```bash
node tools/check_wasm_node.mjs
node tools/check_wasm_views.mjs
.venv/bin/python tools/wasm_full_field_metrics.py --width 64 --height 64 --steps 100
```

### Benchmark Forward Performance

Native Rust:

```bash
cargo run --release --bin bench_forward -- --grids 128,256,512 --steps 500 --trials 5
```

Scalar JavaScript:

```bash
node tools/bench_forward_js.mjs --grids 128,256,512 --steps 500 --trials 5
```

Scalar WASM:

```bash
node tools/bench_forward_wasm.mjs --grids 128,256,512 --steps 500 --trials 5
```

SIMD WASM:

```bash
bash tools/build_wasm_node_simd.sh
node tools/check_wasm_simd.mjs
node tools/bench_forward_wasm_simd.mjs --grids 128,256,512 --steps 500 --trials 5
```

### Run Browser Render Benchmark

```bash
bash tools/build_wasm_web.sh
python3 -m http.server 8000
```

Then open:

```text
http://localhost:8000/www/render_bench.html
```

Repeatable headless run:

```bash
node tools/run_browser_render_bench.mjs --grid 512 --frames 300 --steps 250
```

Manual browser-check procedure:

- [docs/manualcheck-browser-render.md](docs/manualcheck-browser-render.md)

### Run Browser Inverse Recovery

```bash
bash tools/build_wasm_web.sh
python3 -m http.server 8000
```

Then open:

```text
http://localhost:8000/www/inverse.html
```

The heavy optimizer call runs in `www/inverse_worker.js`, not on the main page
thread.

Repeatable headless run:

```bash
node tools/run_browser_inverse_bench.mjs --grid 64 --steps 100 --iterations 8
```

### Run Inverse-Recovery CLI Experiments

Grid-search baseline:

```bash
cargo run --release --bin inverse_grid -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --feed-min 0.058 --feed-max 0.063 --feed-count 11 \
  --kill-min 0.060 --kill-max 0.065 --kill-count 11
```

Multi-regime recovery:

```bash
cargo run --release --bin inverse_regimes -- \
  --width 64 --height 64 --steps 100 \
  --feed-min 0.045 --feed-max 0.070 --feed-count 51 \
  --kill-min 0.055 --kill-max 0.070 --kill-count 31
```

Noise sweep:

```bash
cargo run --release --bin inverse_noise -- \
  --width 64 --height 64 --steps 100 \
  --noise-levels 0.000,0.001,0.005,0.010,0.020,0.050,0.100 \
  --seeds 24301,24589,51966,48879 \
  --feed-min 0.045 --feed-max 0.070 --feed-count 51 \
  --kill-min 0.055 --kill-max 0.070 --kill-count 31
```

AD optimizer vs grid search under noise:

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

### Run Gradient Checks And Overhead Benchmarks

Finite-difference gradient baseline:

```bash
cargo run --release --bin inverse_grad -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Forward-mode AD gradient check:

```bash
cargo run --release --bin inverse_ad -- \
  --width 64 --height 64 --steps 100 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Inverse-gradient overhead:

```bash
cargo run --release --bin bench_inverse -- \
  --grids 64,128,256 --steps 100 --trials 7 \
  --target-feed 0.06055 --target-kill 0.06245 \
  --guess-feed 0.060 --guess-kill 0.063 \
  --epsilon 0.0001
```

Criterion benchmark:

```bash
cargo bench --bench inverse_overhead
```

## Documentation

- [docs/README.md](docs/README.md)
- [docs/reproducibility.md](docs/reproducibility.md)
- [docs/experiment-log.md](docs/experiment-log.md)
- [docs/manualcheck-browser-render.md](docs/manualcheck-browser-render.md)
- [docs/plan.md](docs/plan.md)
- [docs/research-directions.md](docs/research-directions.md)
- [CONTRIBUTING.md](CONTRIBUTING.md)

## Learning Site

A chapter-based Docusaurus site for students and first-time readers lives in
`learn/`.

Use Node 22 for the learning site. If you use `nvm`:

```bash
nvm use
```

Local development:

```bash
cd learn
npm install
npm run start
```

Production build:

```bash
cd learn
npm run build
```

## Notes

- The browser render numbers include both interactive Chrome measurements and
  headless Chrome measurements. Keep those tables separate.
- The browser inverse timings currently come from headless Chrome on a single
  machine. Interactive and cross-browser checks are still optional follow-up
  work, not missing implementation.
