#!/usr/bin/env node
import { createRequire } from "node:module";
import { performance } from "node:perf_hooks";

const require = createRequire(import.meta.url);
const { WasmGrayScott } = require("../pkg-node/grayscott_wasm.js");

function parseArgs() {
  const args = {
    grids: [128, 256, 512],
    trials: 1000,
    radius: 5,
    steps: 100,
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
      case "--trials":
        args.trials = Number.parseInt(value, 10);
        break;
      case "--steps":
        args.steps = Number.parseInt(value, 10);
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

function createSim(grid, args) {
  const sim = new WasmGrayScott(grid, grid);
  sim.seed_square(Math.floor(grid / 2), Math.floor(grid / 2), args.radius);
  sim.run(args.steps);
  return sim;
}

function time(fn, trials) {
  let checksum = 0.0;
  const start = performance.now();
  for (let trial = 0; trial < trials; trial += 1) {
    checksum += fn();
  }
  return {
    msPerTrial: (performance.now() - start) / trials,
    checksum,
  };
}

function sample(values) {
  return values[0] + values[Math.floor(values.length / 2)] + values[values.length - 1];
}

function runGrid(grid, args) {
  const sim = createSim(grid, args);

  const copy = time(() => sample(sim.u_values()) + sample(sim.v_values()), args.trials);
  const view = time(() => sample(sim.u_view()) + sample(sim.v_view()), args.trials);

  return {
    grid,
    len: sim.len(),
    trials: args.trials,
    copyMs: copy.msPerTrial,
    viewMs: view.msPerTrial,
    speedup: copy.msPerTrial / view.msPerTrial,
    copyChecksum: copy.checksum,
    viewChecksum: view.checksum,
  };
}

function printMarkdown(rows) {
  console.log(
    "| Grid | Cells | Trials | Copy ms/trial | View ms/trial | View speedup | Copy checksum | View checksum |",
  );
  console.log("|---|---:|---:|---:|---:|---:|---:|---:|");
  for (const row of rows) {
    console.log(
      `| ${row.grid}x${row.grid} | ${row.len} | ${row.trials} | `
        + `${row.copyMs.toFixed(6)} | ${row.viewMs.toFixed(6)} | `
        + `${row.speedup.toFixed(2)}x | ${row.copyChecksum.toFixed(6)} | `
        + `${row.viewChecksum.toFixed(6)} |`,
    );
  }
}

const args = parseArgs();
printMarkdown(args.grids.map((grid) => runGrid(grid, args)));
