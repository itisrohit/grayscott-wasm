#!/usr/bin/env python3
"""Compute full-field Rust-vs-NumPy validation metrics."""

from __future__ import annotations

import argparse
import json
import math
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "reference"))

import reference_numpy  # noqa: E402


def metrics(actual: np.ndarray, expected: np.ndarray) -> dict[str, float]:
    delta = actual.astype(np.float64) - expected.astype(np.float64)
    abs_delta = np.abs(delta)
    return {
        "mae": float(abs_delta.mean()),
        "rmse": float(math.sqrt(float((delta * delta).mean()))),
        "max_error": float(abs_delta.max()),
    }


def run_case(width: int, height: int, steps: int, radius: int) -> dict[str, object]:
    with tempfile.TemporaryDirectory(prefix="grayscott-fields-") as temp:
        output_dir = Path(temp)
        try:
            subprocess.run(
                [
                    "cargo",
                    "run",
                    "--quiet",
                    "--example",
                    "export_fields",
                    "--",
                    "--width",
                    str(width),
                    "--height",
                    str(height),
                    "--steps",
                    str(steps),
                    "--radius",
                    str(radius),
                    "--output-dir",
                    str(output_dir),
                ],
                cwd=ROOT,
                check=True,
                text=True,
                capture_output=True,
            )
        except subprocess.CalledProcessError as err:
            raise RuntimeError(
                "Rust field export failed\n"
                f"command: {' '.join(err.cmd)}\n"
                f"stdout:\n{err.stdout}\n"
                f"stderr:\n{err.stderr}"
            ) from err

        rust_u = np.fromfile(output_dir / "u_f32_le.raw", dtype="<f4").reshape((height, width))
        rust_v = np.fromfile(output_dir / "v_f32_le.raw", dtype="<f4").reshape((height, width))

    args = argparse.Namespace(
        width=width,
        height=height,
        steps=steps,
        radius=radius,
        feed=0.060,
        kill=0.062,
        diff_u=0.16,
        diff_v=0.08,
        dt=1.0,
    )
    numpy_u, numpy_v = reference_numpy.run(args)

    return {
        "width": width,
        "height": height,
        "steps": steps,
        "radius": radius,
        "u": metrics(rust_u, numpy_u),
        "v": metrics(rust_v, numpy_v),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--width", type=int, default=64)
    parser.add_argument("--height", type=int, default=64)
    parser.add_argument("--steps", type=int, nargs="+", default=[100, 500, 1000])
    parser.add_argument("--radius", type=int, default=5)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()

    rows = [run_case(args.width, args.height, steps, args.radius) for steps in args.steps]

    if args.json:
        print(json.dumps(rows, indent=2, sort_keys=True))
        return

    print("| Grid | Steps | u_MAE | v_MAE | u_RMSE | v_RMSE | u_MaxErr | v_MaxErr |")
    print("|---|---:|---:|---:|---:|---:|---:|---:|")
    for row in rows:
        grid = f"{row['width']}x{row['height']}"
        print(
            f"| {grid} | {row['steps']} | "
            f"{row['u']['mae']:.3e} | {row['v']['mae']:.3e} | "
            f"{row['u']['rmse']:.3e} | {row['v']['rmse']:.3e} | "
            f"{row['u']['max_error']:.3e} | {row['v']['max_error']:.3e} |"
        )


if __name__ == "__main__":
    main()
