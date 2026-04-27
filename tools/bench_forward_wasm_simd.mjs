#!/usr/bin/env node
import { createRequire } from "node:module";
import { performance } from "node:perf_hooks";

const require = createRequire(import.meta.url);
const { WasmGrayScott } = require("../pkg-node-simd/grayscott_wasm.js");

function parseArgs() {
  const args = {
    grids: [128, 256, 512],
    steps: 500,
    trials: 5,
    warmupSteps: 25,
    radius: 5,
  };

  const argv = process.argv.slice(2);
  for (let i = 0; i < argv.length; i += 2) {
    const flag = argv[i];
    const value = argv[i + 1];
    if (value === undefined) {
      throw new Error(`missing value for ${flag}`);
    }

    switch (flag) {
      case "--grids":
        args.grids = value.split(",").map((part) => Number.parseInt(part, 10));
        break;
      case "--steps":
        args.steps = Number.parseInt(value, 10);
        break;
      case "--trials":
        args.trials = Number.parseInt(value, 10);
        break;
      case "--warmup-steps":
        args.warmupSteps = Number.parseInt(value, 10);
        break;
      case "--radius":
        args.radius = Number.parseInt(value, 10);
        break;
      default:
        throw new Error(`unknown argument: ${flag}`);
    }
  }

  if (args.grids.length === 0 || args.grids.some((grid) => !Number.isInteger(grid) || grid <= 0)) {
    throw new Error("--grids must contain positive integers");
  }
  if (args.steps <= 0 || args.trials <= 0 || args.warmupSteps < 0 || args.radius < 0) {
    throw new Error("--steps and --trials must be positive; warmup/radius must be non-negative");
  }

  return args;
}

function createSim(grid, radius) {
  const sim = new WasmGrayScott(grid, grid);
  sim.seed_square(Math.floor(grid / 2), Math.floor(grid / 2), radius);
  return sim;
}

function median(values) {
  values.sort((a, b) => a - b);
  const mid = Math.floor(values.length / 2);
  return values.length % 2 === 0 ? (values[mid - 1] + values[mid]) / 2.0 : values[mid];
}

function runOnce(grid, args, method) {
  const sim = createSim(grid, args.radius);
  const start = performance.now();
  if (method === "scalar") {
    sim.run(args.steps);
  } else {
    sim.run_simd(args.steps);
  }
  return {
    msPerStep: (performance.now() - start) / args.steps,
    checksum: sim.checksum(),
  };
}

function summarize(grid, args, method) {
  const warmup = createSim(grid, args.radius);
  if (method === "scalar") {
    warmup.run(args.warmupSteps);
  } else {
    warmup.run_simd(args.warmupSteps);
  }
  warmup.checksum();

  const msPerStep = [];
  let checksum = 0.0;
  for (let trial = 0; trial < args.trials; trial += 1) {
    const result = runOnce(grid, args, method);
    msPerStep.push(result.msPerStep);
    checksum = result.checksum;
  }

  return {
    median: median(msPerStep),
    min: Math.min(...msPerStep),
    max: Math.max(...msPerStep),
    checksum,
  };
}

function printMarkdown(rows) {
  console.log(
    "| Grid | Steps | Trials | Scalar ms/step | SIMD ms/step | SIMD speedup | Scalar checksum | SIMD checksum |",
  );
  console.log("|---|---:|---:|---:|---:|---:|---:|---:|");
  for (const row of rows) {
    console.log(
      `| ${row.grid}x${row.grid} | ${row.steps} | ${row.trials} | `
        + `${row.scalar.median.toFixed(6)} | ${row.simd.median.toFixed(6)} | `
        + `${(row.scalar.median / row.simd.median).toFixed(2)}x | `
        + `${row.scalar.checksum.toFixed(6)} | ${row.simd.checksum.toFixed(6)} |`,
    );
  }
}

const args = parseArgs();
const probe = createSim(8, 1);
if (!probe.simd_enabled()) {
  throw new Error("SIMD package was not built with wasm32 simd128 enabled");
}

printMarkdown(
  args.grids.map((grid) => ({
    grid,
    steps: args.steps,
    trials: args.trials,
    scalar: summarize(grid, args, "scalar"),
    simd: summarize(grid, args, "simd"),
  })),
);
