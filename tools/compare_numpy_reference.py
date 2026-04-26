#!/usr/bin/env python3
"""Compare Rust scalar summary against the NumPy float32 reference."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "reference"))

import reference_numpy


LINE_RE = re.compile(
    r"(?P<field>[uv])_min=(?P<min>[-+0-9.eE]+) "
    r"(?P=field)_max=(?P<max>[-+0-9.eE]+) "
    r"(?P=field)_mean=(?P<mean>[-+0-9.eE]+)"
)


def parse_rust_summary(output: str) -> dict[str, dict[str, float]]:
    parsed: dict[str, dict[str, float]] = {}
    for line in output.splitlines():
        match = LINE_RE.search(line)
        if match:
            parsed[match.group("field")] = {
                "min": float(match.group("min")),
                "max": float(match.group("max")),
                "mean": float(match.group("mean")),
            }
    if set(parsed) != {"u", "v"}:
        raise RuntimeError(f"could not parse Rust summary output:\n{output}")
    return parsed


def main() -> None:
    rust = subprocess.run(
        ["cargo", "run", "--quiet", "--example", "summary"],
        cwd=ROOT,
        check=True,
        text=True,
        capture_output=True,
    )
    rust_stats = parse_rust_summary(rust.stdout)

    args = argparse.Namespace(
        width=64,
        height=64,
        steps=100,
        radius=5,
        feed=0.060,
        kill=0.062,
        diff_u=0.16,
        diff_v=0.08,
        dt=1.0,
    )
    u, v = reference_numpy.run(args)
    numpy_stats = {
        "u": {"min": float(u.min()), "max": float(u.max()), "mean": float(u.mean())},
        "v": {"min": float(v.min()), "max": float(v.max()), "mean": float(v.mean())},
    }

    tolerance = 1.0e-6
    for field in ("u", "v"):
        for metric in ("min", "max", "mean"):
            delta = abs(rust_stats[field][metric] - numpy_stats[field][metric])
            if delta > tolerance:
                raise SystemExit(
                    f"{field}_{metric} mismatch: "
                    f"rust={rust_stats[field][metric]} "
                    f"numpy={numpy_stats[field][metric]} "
                    f"delta={delta} tolerance={tolerance}"
                )

    print("Rust scalar summary matches NumPy float32 reference within 1e-6.")


if __name__ == "__main__":
    main()
