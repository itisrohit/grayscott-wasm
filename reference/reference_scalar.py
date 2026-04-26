#!/usr/bin/env python3
"""Dependency-free scalar reference implementation for Gray-Scott.

This is slower than the NumPy reference but useful for environments where NumPy
is not installed. It mirrors the Rust scalar implementation exactly: f32-like
inputs, periodic boundaries, 5-point Laplacian, and explicit Euler stepping.
Python floats are f64, so use this as a logic reference, not bitwise parity.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def index(width: int, x: int, y: int) -> int:
    return y * width + x


def seed_square(
    width: int,
    height: int,
    u: list[float],
    v: list[float],
    center_x: int,
    center_y: int,
    radius: int,
) -> None:
    min_x = max(center_x - radius, 0)
    max_x = min(center_x + radius, width - 1)
    min_y = max(center_y - radius, 0)
    max_y = min(center_y + radius, height - 1)
    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            i = index(width, x, y)
            u[i] = 0.50
            v[i] = 0.25


def step(
    width: int,
    height: int,
    u: list[float],
    v: list[float],
    feed: float,
    kill: float,
    diff_u: float,
    diff_v: float,
    dt: float,
) -> tuple[list[float], list[float]]:
    next_u = [0.0] * len(u)
    next_v = [0.0] * len(v)
    for y in range(height):
        y_up = height - 1 if y == 0 else y - 1
        y_down = 0 if y + 1 == height else y + 1
        for x in range(width):
            x_left = width - 1 if x == 0 else x - 1
            x_right = 0 if x + 1 == width else x + 1

            center = index(width, x, y)
            left = index(width, x_left, y)
            right = index(width, x_right, y)
            up = index(width, x, y_up)
            down = index(width, x, y_down)

            u_c = u[center]
            v_c = v[center]
            lap_u = u[left] + u[right] + u[up] + u[down] - 4.0 * u_c
            lap_v = v[left] + v[right] + v[up] + v[down] - 4.0 * v_c
            uvv = u_c * v_c * v_c
            next_u[center] = u_c + dt * (diff_u * lap_u - uvv + feed * (1.0 - u_c))
            next_v[center] = v_c + dt * (diff_v * lap_v + uvv - (feed + kill) * v_c)
    return next_u, next_v


def stats(values: list[float]) -> dict[str, float]:
    return {
        "min": min(values),
        "max": max(values),
        "mean": sum(values) / len(values),
    }


def run(args: argparse.Namespace) -> tuple[list[float], list[float]]:
    u = [1.0] * (args.width * args.height)
    v = [0.0] * (args.width * args.height)
    seed_square(args.width, args.height, u, v, args.width // 2, args.height // 2, args.radius)

    for _ in range(args.steps):
        u, v = step(
            args.width,
            args.height,
            u,
            v,
            args.feed,
            args.kill,
            args.diff_u,
            args.diff_v,
            args.dt,
        )
    return u, v


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--width", type=int, default=64)
    parser.add_argument("--height", type=int, default=64)
    parser.add_argument("--steps", type=int, default=100)
    parser.add_argument("--radius", type=int, default=5)
    parser.add_argument("--feed", type=float, default=0.060)
    parser.add_argument("--kill", type=float, default=0.062)
    parser.add_argument("--diff-u", type=float, default=0.16)
    parser.add_argument("--diff-v", type=float, default=0.08)
    parser.add_argument("--dt", type=float, default=1.0)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    u, v = run(args)
    payload = {"u": stats(u), "v": stats(v)}
    print(json.dumps(payload, indent=2, sort_keys=True))

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps({"u": u, "v": v}, separators=(",", ":")))


if __name__ == "__main__":
    main()
