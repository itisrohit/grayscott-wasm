#!/usr/bin/env node
import { createRequire } from "node:module";
import { performance } from "node:perf_hooks";

const require = createRequire(import.meta.url);
const { WasmGrayScott } = require("../pkg-node/grayscott_wasm.js");

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

function runGrid(grid, args) {
  const warmup = createSim(grid, args.radius);
  warmup.run(args.warmupSteps);
  warmup.checksum();

  const msPerStep = [];
  let lastChecksum = 0.0;

  for (let trial = 0; trial < args.trials; trial += 1) {
    const sim = createSim(grid, args.radius);
    const start = performance.now();
    sim.run(args.steps);
    const elapsed = performance.now() - start;
    lastChecksum = sim.checksum();
    msPerStep.push(elapsed / args.steps);
  }

  const minMsPerStep = Math.min(...msPerStep);
  const maxMsPerStep = Math.max(...msPerStep);
  const medianMsPerStep = median(msPerStep);
  const medianStepsPerSec = 1000.0 / medianMsPerStep;
  const cellsPerSec = medianStepsPerSec * grid * grid;

  return {
    grid,
    steps: args.steps,
    trials: args.trials,
    medianMsPerStep,
    minMsPerStep,
    maxMsPerStep,
    medianStepsPerSec,
    cellsPerSec,
    checksum: lastChecksum,
  };
}

function printMarkdown(rows) {
  console.log(
    "| Grid | Steps | Trials | Median ms/step | Min ms/step | Max ms/step | Median steps/s | Cells/s | Checksum |",
  );
  console.log("|---|---:|---:|---:|---:|---:|---:|---:|---:|");
  for (const row of rows) {
    console.log(
      `| ${row.grid}x${row.grid} | ${row.steps} | ${row.trials} | `
        + `${row.medianMsPerStep.toFixed(6)} | ${row.minMsPerStep.toFixed(6)} | `
        + `${row.maxMsPerStep.toFixed(6)} | ${row.medianStepsPerSec.toFixed(2)} | `
        + `${row.cellsPerSec.toExponential(3)} | ${row.checksum.toFixed(6)} |`,
    );
  }
}

const args = parseArgs();
printMarkdown(args.grids.map((grid) => runGrid(grid, args)));
