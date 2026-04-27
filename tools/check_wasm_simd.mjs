#!/usr/bin/env node
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { WasmGrayScott } = require("../pkg-node-simd/grayscott_wasm.js");

function createSim(grid, radius) {
  const sim = new WasmGrayScott(grid, grid);
  sim.seed_square(Math.floor(grid / 2), Math.floor(grid / 2), radius);
  return sim;
}

function maxAbsDelta(left, right) {
  if (left.length !== right.length) {
    throw new Error(`length mismatch: ${left.length} !== ${right.length}`);
  }
  let max = 0.0;
  for (let i = 0; i < left.length; i += 1) {
    const delta = Math.abs(left[i] - right[i]);
    if (delta > max) {
      max = delta;
    }
  }
  return max;
}

const grid = 128;
const steps = 250;
const radius = 5;
const scalar = createSim(grid, radius);
const simd = createSim(grid, radius);

if (!simd.simd_enabled()) {
  throw new Error("SIMD package was not built with wasm32 simd128 enabled");
}

scalar.run(steps);
simd.run_simd(steps);

const uDelta = maxAbsDelta(scalar.u_values(), simd.u_values());
const vDelta = maxAbsDelta(scalar.v_values(), simd.v_values());
const checksumDelta = Math.abs(scalar.checksum() - simd.checksum());
const tolerance = 2.0e-6;

if (uDelta > tolerance || vDelta > tolerance || checksumDelta > tolerance * grid * grid) {
  throw new Error(
    `SIMD mismatch: uDelta=${uDelta}, vDelta=${vDelta}, checksumDelta=${checksumDelta}`,
  );
}

console.log(
  `WASM SIMD matches scalar: uMax=${uDelta.toExponential(3)}, `
    + `vMax=${vDelta.toExponential(3)}, checksumDelta=${checksumDelta.toExponential(3)}`,
);
