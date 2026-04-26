#!/usr/bin/env bash
set -euo pipefail

export PRE_COMMIT_HOME="${PRE_COMMIT_HOME:-.pre-commit-cache}"

.venv/bin/pre-commit install
.venv/bin/pre-commit install --hook-type pre-push
