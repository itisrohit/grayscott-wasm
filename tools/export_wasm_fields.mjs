#!/usr/bin/env node
import { createRequire } from "node:module";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";

const require = createRequire(import.meta.url);
const { WasmGrayScott } = require("../pkg-node/grayscott_wasm.js");

function parseArgs() {
  const args = {
    width: 64,
    height: 64,
    steps: 100,
    radius: 5,
    feed: 0.06,
    kill: 0.062,
    diffU: 0.16,
    diffV: 0.08,
    dt: 1.0,
    outputDir: "data/wasm_fields",
  };

  const argv = process.argv.slice(2);
  for (let i = 0; i < argv.length; i += 2) {
    const flag = argv[i];
    const value = argv[i + 1];
    if (value === undefined) {
      throw new Error(`missing value for ${flag}`);
    }

    switch (flag) {
      case "--width":
        args.width = Number.parseInt(value, 10);
        break;
      case "--height":
        args.height = Number.parseInt(value, 10);
        break;
      case "--steps":
        args.steps = Number.parseInt(value, 10);
        break;
      case "--radius":
        args.radius = Number.parseInt(value, 10);
        break;
      case "--feed":
        args.feed = Number.parseFloat(value);
        break;
      case "--kill":
        args.kill = Number.parseFloat(value);
        break;
      case "--diff-u":
        args.diffU = Number.parseFloat(value);
        break;
      case "--diff-v":
        args.diffV = Number.parseFloat(value);
        break;
      case "--dt":
        args.dt = Number.parseFloat(value);
        break;
      case "--output-dir":
        args.outputDir = value;
        break;
      default:
        throw new Error(`unknown argument: ${flag}`);
    }
  }

  return args;
}

function f32Raw(values) {
  const array = Float32Array.from(values);
  const buffer = Buffer.alloc(array.length * 4);
  for (let i = 0; i < array.length; i += 1) {
    buffer.writeFloatLE(array[i], i * 4);
  }
  return buffer;
}

const args = parseArgs();
const sim = new WasmGrayScott(args.width, args.height);
sim.set_params(args.feed, args.kill, args.diffU, args.diffV, args.dt);
sim.seed_square(Math.floor(args.width / 2), Math.floor(args.height / 2), args.radius);
sim.run(args.steps);

await mkdir(args.outputDir, { recursive: true });
await writeFile(join(args.outputDir, "u_f32_le.raw"), f32Raw(sim.u_values()));
await writeFile(join(args.outputDir, "v_f32_le.raw"), f32Raw(sim.v_values()));
await writeFile(
  join(args.outputDir, "metadata.json"),
  `${JSON.stringify(
    {
      width: args.width,
      height: args.height,
      steps: args.steps,
      radius: args.radius,
      feed: args.feed,
      kill: args.kill,
      diff_u: args.diffU,
      diff_v: args.diffV,
      dt: args.dt,
      dtype: "f32_le",
      u: "u_f32_le.raw",
      v: "v_f32_le.raw",
      checksum: sim.checksum(),
    },
    null,
    2,
  )}\n`,
);

console.log(args.outputDir);
