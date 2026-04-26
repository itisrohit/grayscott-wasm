#!/usr/bin/env node
import { createRequire } from "node:module";
import { performance } from "node:perf_hooks";

const require = createRequire(import.meta.url);
const { WasmGrayScott } = require("../pkg-node/grayscott_wasm.js");

function parseArgs() {
  const args = {
    grids: [64, 128, 256],
    steps: 500,
    trials: 7,
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
      case "--radius":
        args.radius = Number.parseInt(value, 10);
        break;
      default:
        throw new Error(`unknown argument: ${flag}`);
    }
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

function timeBulk(grid, args) {
  const values = [];
  let checksum = 0.0;
  for (let trial = 0; trial < args.trials; trial += 1) {
    const sim = createSim(grid, args.radius);
    const start = performance.now();
    sim.run(args.steps);
    const elapsed = performance.now() - start;
    checksum = sim.checksum();
    values.push(elapsed / args.steps);
  }
  return { msPerStep: median(values), checksum };
}

function timeStepCalls(grid, args) {
  const values = [];
  let checksum = 0.0;
  for (let trial = 0; trial < args.trials; trial += 1) {
    const sim = createSim(grid, args.radius);
    const start = performance.now();
    for (let step = 0; step < args.steps; step += 1) {
      sim.step();
    }
    const elapsed = performance.now() - start;
    checksum = sim.checksum();
    values.push(elapsed / args.steps);
  }
  return { msPerStep: median(values), checksum };
}

function printMarkdown(rows) {
  console.log(
    "| Grid | Steps | Trials | Bulk ms/step | Per-step-call ms/step | Boundary overhead | Bulk checksum | Step checksum |",
  );
  console.log("|---|---:|---:|---:|---:|---:|---:|---:|");
  for (const row of rows) {
    console.log(
      `| ${row.grid}x${row.grid} | ${row.steps} | ${row.trials} | `
        + `${row.bulkMsPerStep.toFixed(6)} | ${row.stepMsPerStep.toFixed(6)} | `
        + `${row.overhead.toFixed(2)}x | ${row.bulkChecksum.toFixed(6)} | `
        + `${row.stepChecksum.toFixed(6)} |`,
    );
  }
}

const args = parseArgs();
const rows = args.grids.map((grid) => {
  const bulk = timeBulk(grid, args);
  const step = timeStepCalls(grid, args);
  return {
    grid,
    steps: args.steps,
    trials: args.trials,
    bulkMsPerStep: bulk.msPerStep,
    stepMsPerStep: step.msPerStep,
    overhead: step.msPerStep / bulk.msPerStep,
    bulkChecksum: bulk.checksum,
    stepChecksum: step.checksum,
  };
});

printMarkdown(rows);
