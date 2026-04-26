#!/usr/bin/env node
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { WasmGrayScott } = require("../pkg-node/grayscott_wasm.js");

function maxAbsDiff(a, b) {
  if (a.length !== b.length) {
    throw new Error(`length mismatch: ${a.length} != ${b.length}`);
  }

  let max = 0.0;
  for (let i = 0; i < a.length; i += 1) {
    max = Math.max(max, Math.abs(a[i] - b[i]));
  }
  return max;
}

const sim = new WasmGrayScott(64, 64);
sim.seed_square(32, 32, 5);
sim.run(100);

const uView = sim.u_view();
const vView = sim.v_view();
const uCopy = sim.u_values();
const vCopy = sim.v_values();

const uMax = maxAbsDiff(uView, uCopy);
const vMax = maxAbsDiff(vView, vCopy);

if (uMax !== 0 || vMax !== 0) {
  throw new Error(`WASM view mismatch: u_max=${uMax}, v_max=${vMax}`);
}

console.log("WASM zero-copy views match copied fields exactly.");
