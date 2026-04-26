#!/usr/bin/env node
import { createRequire } from "node:module";
import { performance } from "node:perf_hooks";

const require = createRequire(import.meta.url);
const { WasmGrayScott } = require("../pkg-node/grayscott_wasm.js");

function parseArgs() {
  const args = {
    grids: [128, 256, 512],
    trials: 200,
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

function fieldToGrayscaleRgba(field, pixels) {
  for (let i = 0, p = 0; i < field.length; i += 1, p += 4) {
    const value = Math.max(0, Math.min(255, Math.round(field[i] * 255)));
    pixels[p] = value;
    pixels[p + 1] = value;
    pixels[p + 2] = value;
    pixels[p + 3] = 255;
  }
}

function checksum(pixels) {
  let sum = 0;
  const stride = Math.max(1, Math.floor(pixels.length / 64));
  for (let i = 0; i < pixels.length; i += stride) {
    sum += pixels[i];
  }
  return sum;
}

function time(fn, trials) {
  let total = 0;
  const start = performance.now();
  for (let trial = 0; trial < trials; trial += 1) {
    total += fn();
  }
  return {
    msPerTrial: (performance.now() - start) / trials,
    checksum: total,
  };
}

function runGrid(grid, args) {
  const sim = createSim(grid, args);
  const field = sim.u_view();
  const reusablePixels = new Uint8ClampedArray(field.length * 4);

  const reuse = time(() => {
    fieldToGrayscaleRgba(field, reusablePixels);
    return checksum(reusablePixels);
  }, args.trials);

  const allocate = time(() => {
    const pixels = new Uint8ClampedArray(field.length * 4);
    fieldToGrayscaleRgba(field, pixels);
    return checksum(pixels);
  }, args.trials);

  return {
    grid,
    cells: field.length,
    trials: args.trials,
    reuseMs: reuse.msPerTrial,
    allocateMs: allocate.msPerTrial,
    allocateOverhead: allocate.msPerTrial / reuse.msPerTrial,
    reuseChecksum: reuse.checksum,
    allocateChecksum: allocate.checksum,
  };
}

function printMarkdown(rows) {
  console.log(
    "| Grid | Cells | Trials | Reuse buffer ms/frame | Allocate buffer ms/frame | Allocation overhead | Reuse checksum | Allocate checksum |",
  );
  console.log("|---|---:|---:|---:|---:|---:|---:|---:|");
  for (const row of rows) {
    console.log(
      `| ${row.grid}x${row.grid} | ${row.cells} | ${row.trials} | `
        + `${row.reuseMs.toFixed(6)} | ${row.allocateMs.toFixed(6)} | `
        + `${row.allocateOverhead.toFixed(2)}x | ${row.reuseChecksum.toFixed(0)} | `
        + `${row.allocateChecksum.toFixed(0)} |`,
    );
  }
}

const args = parseArgs();
printMarkdown(args.grids.map((grid) => runGrid(grid, args)));
