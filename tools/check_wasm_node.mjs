#!/usr/bin/env node
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { WasmGrayScott } = require("../pkg-node/grayscott_wasm.js");

const sim = new WasmGrayScott(64, 64);
sim.seed_square(32, 32, 5);
sim.run(100);

const expected = 4056.932528740907;
const actual = sim.checksum();
const delta = Math.abs(actual - expected);
const tolerance = 1e-6;

if (delta > tolerance) {
  throw new Error(`WASM checksum mismatch: actual=${actual}, expected=${expected}, delta=${delta}`);
}

console.log(`WASM checksum ok: ${actual.toFixed(12)}`);
