#!/usr/bin/env node
import { performance } from "node:perf_hooks";

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
  const len = grid * grid;
  const sim = {
    width: grid,
    height: grid,
    u: new Float32Array(len),
    v: new Float32Array(len),
    nextU: new Float32Array(len),
    nextV: new Float32Array(len),
  };
  sim.u.fill(1.0);
  sim.nextU.fill(1.0);
  seedSquare(sim, Math.floor(grid / 2), Math.floor(grid / 2), radius);
  return sim;
}

function index(width, x, y) {
  return y * width + x;
}

function seedSquare(sim, centerX, centerY, radius) {
  const minX = Math.max(centerX - radius, 0);
  const maxX = Math.min(centerX + radius, sim.width - 1);
  const minY = Math.max(centerY - radius, 0);
  const maxY = Math.min(centerY + radius, sim.height - 1);

  for (let y = minY; y <= maxY; y += 1) {
    for (let x = minX; x <= maxX; x += 1) {
      const i = index(sim.width, x, y);
      sim.u[i] = 0.5;
      sim.v[i] = 0.25;
    }
  }
  sim.nextU.set(sim.u);
  sim.nextV.set(sim.v);
}

function step(sim) {
  const { width, height, u, v, nextU, nextV } = sim;
  const feed = 0.06;
  const kill = 0.062;
  const diffU = 0.16;
  const diffV = 0.08;
  const dt = 1.0;

  for (let y = 0; y < height; y += 1) {
    const yUp = y === 0 ? height - 1 : y - 1;
    const yDown = y + 1 === height ? 0 : y + 1;

    for (let x = 0; x < width; x += 1) {
      const xLeft = x === 0 ? width - 1 : x - 1;
      const xRight = x + 1 === width ? 0 : x + 1;

      const center = y * width + x;
      const left = y * width + xLeft;
      const right = y * width + xRight;
      const up = yUp * width + x;
      const down = yDown * width + x;

      const uValue = u[center];
      const vValue = v[center];
      const lapU = u[left] + u[right] + u[up] + u[down] - 4.0 * uValue;
      const lapV = v[left] + v[right] + v[up] + v[down] - 4.0 * vValue;
      const uvv = uValue * vValue * vValue;

      nextU[center] = uValue + dt * (diffU * lapU - uvv + feed * (1.0 - uValue));
      nextV[center] = vValue + dt * (diffV * lapV + uvv - (feed + kill) * vValue);
    }
  }

  sim.u = nextU;
  sim.v = nextV;
  sim.nextU = u;
  sim.nextV = v;
}

function run(sim, steps) {
  for (let i = 0; i < steps; i += 1) {
    step(sim);
  }
}

function checksum(sim) {
  let sum = 0.0;
  for (const value of sim.u) {
    sum += value;
  }
  for (const value of sim.v) {
    sum += value;
  }
  return sum;
}

function median(values) {
  values.sort((a, b) => a - b);
  const mid = Math.floor(values.length / 2);
  return values.length % 2 === 0 ? (values[mid - 1] + values[mid]) / 2.0 : values[mid];
}

function runGrid(grid, args) {
  const warmup = createSim(grid, args.radius);
  run(warmup, args.warmupSteps);
  checksum(warmup);

  const msPerStep = [];
  let lastChecksum = 0.0;

  for (let trial = 0; trial < args.trials; trial += 1) {
    const sim = createSim(grid, args.radius);
    const start = performance.now();
    run(sim, args.steps);
    const elapsed = performance.now() - start;
    lastChecksum = checksum(sim);
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
