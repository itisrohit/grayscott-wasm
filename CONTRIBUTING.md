# Contributing

This repository is a research artifact first and a software project second. The
main expectation is that changes stay reproducible, measured, and easy to
verify.

## Before You Change Anything

Read these files first:

1. [README.md](README.md)
2. [docs/README.md](docs/README.md)
3. [docs/experiment-log.md](docs/experiment-log.md)

If your change affects paper numbers, browser measurements, or benchmark output,
update the relevant documentation in the same change.

## Setup

Create the Python environment:

```bash
uv sync
```

Install local git hooks:

```bash
bash tools/install-hooks.sh
```

## Required Checks

Run the full gate before opening a PR or asking for review:

```bash
bash tools/quality.sh
```

That script currently runs:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `ruff format --check`
- `ruff check`
- JavaScript syntax checks
- Node.js WASM build and checks
- browser WASM build
- Rust/Python/NumPy/WASM validation scripts

## Change Rules

- Keep edits scoped.
- Do not change measured values in docs unless you reran the relevant command.
- If a benchmark table changes, record the command and observed result in
  [docs/experiment-log.md](docs/experiment-log.md).
- If browser benchmark behavior changes, update
  [docs/manualcheck-browser-render.md](docs/manualcheck-browser-render.md) when
  needed.
- If paper claims change, keep [paper/main.tex](paper/main.tex) and the docs in
  sync.

## Code Style

- Rust: pass `cargo fmt` and `cargo clippy`.
- Python: pass `ruff format` and `ruff check`.
- JavaScript: pass `bash tools/check_js.sh`.
- Prefer simple, explicit code over clever abstractions.

## Commits

- Use clear commit messages tied to the actual change.
- Keep generated benchmark artifacts out of the repo unless they are already
  tracked and intentionally updated.

## Reproducibility

If your change affects the paper or benchmark claims, also check:

- [docs/reproducibility.md](docs/reproducibility.md)
- [paper/grayscott_wasm_IEEE_Journal_Paper.pdf](paper/grayscott_wasm_IEEE_Journal_Paper.pdf)

The baseline standard is simple: another person should be able to see what you
changed, rerun it, and understand which paper/log lines moved.
