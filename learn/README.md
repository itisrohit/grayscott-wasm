# Gray-Scott Learning Site

This directory contains the Docusaurus site for students and first-time
readers.

## What it is for

Use this site when you want the project explained chapter by chapter:

- what the Gray-Scott system is,
- what the research question was,
- what the experiments measured,
- what terms like AD, SIMD, WASM, and Web Worker mean,
- how to reproduce the artifact.

Use `docs/` when you want the raw experiment log, paper-facing notes, and
artifact checklists.

## Local development

Use Node 22:

```bash
nvm use
```

Install dependencies:

```bash
cd learn
npm install
```

Start the local site:

```bash
npm run start
```

Build the static site:

```bash
npm run build
```

## Deployment

GitHub Pages deployment is defined in:

- `../.github/workflows/learn-site.yml`

The published site path is expected to be:

- `https://itisrohit.github.io/grayscott-wasm/`
