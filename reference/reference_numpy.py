#!/usr/bin/env python3
"""Float32 NumPy reference implementation for the Gray-Scott solver.

This script is intentionally simple and slow-ish. It exists to validate the Rust
solver, not to win performance benchmarks.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import numpy as np


def seed_square(u: np.ndarray, v: np.ndarray, center_x: int, center_y: int, radius: int) -> None:
    min_x = max(center_x - radius, 0)
    max_x = min(center_x + radius, u.shape[1] - 1)
    min_y = max(center_y - radius, 0)
    max_y = min(center_y + radius, u.shape[0] - 1)
    u[min_y : max_y + 1, min_x : max_x + 1] = np.float32(0.50)
    v[min_y : max_y + 1, min_x : max_x + 1] = np.float32(0.25)


def step(
    u: np.ndarray,
    v: np.ndarray,
    feed: np.float32,
    kill: np.float32,
    diff_u: np.float32,
    diff_v: np.float32,
    dt: np.float32,
) -> tuple[np.ndarray, np.ndarray]:
    lap_u = (
        np.roll(u, 1, axis=0)
        + np.roll(u, -1, axis=0)
        + np.roll(u, 1, axis=1)
        + np.roll(u, -1, axis=1)
        - np.float32(4.0) * u
    )
    lap_v = (
        np.roll(v, 1, axis=0)
        + np.roll(v, -1, axis=0)
        + np.roll(v, 1, axis=1)
        + np.roll(v, -1, axis=1)
        - np.float32(4.0) * v
    )
    uvv = u * v * v
    next_u = u + dt * (diff_u * lap_u - uvv + feed * (np.float32(1.0) - u))
    next_v = v + dt * (diff_v * lap_v + uvv - (feed + kill) * v)
    return next_u.astype(np.float32), next_v.astype(np.float32)


def run(args: argparse.Namespace) -> tuple[np.ndarray, np.ndarray]:
    u = np.ones((args.height, args.width), dtype=np.float32)
    v = np.zeros((args.height, args.width), dtype=np.float32)
    seed_square(u, v, args.width // 2, args.height // 2, args.radius)

    feed = np.float32(args.feed)
    kill = np.float32(args.kill)
    diff_u = np.float32(args.diff_u)
    diff_v = np.float32(args.diff_v)
    dt = np.float32(args.dt)

    for _ in range(args.steps):
        u, v = step(u, v, feed, kill, diff_u, diff_v, dt)

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
    print(f"u_min={float(u.min()):.9f} u_max={float(u.max()):.9f} u_mean={float(u.mean()):.9f}")
    print(f"v_min={float(v.min()):.9f} v_max={float(v.max()):.9f} v_mean={float(v.mean()):.9f}")

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        np.savez(args.output, u=u, v=v)


if __name__ == "__main__":
    main()
