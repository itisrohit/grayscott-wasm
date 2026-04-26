#!/usr/bin/env bash
set -euo pipefail

mapfile -t js_files < <(find . \
  \( -path './target' -o -path './pkg' -o -path './pkg-node' -o -path './pkg-web' -o -path './.git' -o -path './.venv' -o -path './.uv-cache' -o -path './.pre-commit-cache' \) -prune \
  -o \( -name '*.js' -o -name '*.mjs' \) -print | sort)

if ((${#js_files[@]} == 0)); then
  echo "No JavaScript files found."
  exit 0
fi

for file in "${js_files[@]}"; do
  node --check "$file"
done
