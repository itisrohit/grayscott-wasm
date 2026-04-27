import init, { WasmGrayScott } from "../pkg-web/grayscott_wasm.js";

const runButton = document.querySelector("#run");
const gridInput = document.querySelector("#grid");
const framesInput = document.querySelector("#frames");
const stepsInput = document.querySelector("#steps");
const canvas = document.querySelector("#canvas");
const bitmapCanvas = document.querySelector("#bitmap-canvas");
const resultsBody = document.querySelector("#results");
const statusOutput = document.querySelector("#status");

const ctx = canvas.getContext("2d", { alpha: false });
const bitmapContext = bitmapCanvas.getContext("bitmaprenderer");
let wasmReady = false;

function assertElement(value, name) {
  if (!value) {
    throw new Error(`Missing ${name}`);
  }
  return value;
}

function formatMs(value) {
  return Number.isFinite(value) ? value.toFixed(6) : "unsupported";
}

function setResults(rows) {
  resultsBody.replaceChildren(
    ...rows.map(([name, value]) => {
      const row = document.createElement("tr");
      const metric = document.createElement("td");
      const time = document.createElement("td");
      metric.textContent = name;
      time.textContent = formatMs(value);
      row.append(metric, time);
      return row;
    }),
  );
}

function writeStatus(value) {
  statusOutput.textContent =
    typeof value === "string" ? value : JSON.stringify(value, null, 2);
}

function applyQuerySettings() {
  const params = new URLSearchParams(window.location.search);
  const grid = params.get("grid");
  const frames = params.get("frames");
  const steps = params.get("steps");

  if (grid && [...gridInput.options].some((option) => option.value === grid)) {
    gridInput.value = grid;
  }
  if (frames) {
    framesInput.value = frames;
  }
  if (steps) {
    stepsInput.value = steps;
  }

  return params.get("autorun") === "1";
}

function fillPixels(field, pixels) {
  for (let i = 0, j = 0; i < field.length; i += 1, j += 4) {
    const v = Math.max(0, Math.min(255, Math.round((1 - field[i]) * 255)));
    pixels[j] = v;
    pixels[j + 1] = Math.min(255, v + 36);
    pixels[j + 2] = Math.max(0, 255 - v);
    pixels[j + 3] = 255;
  }
}

function checksumPixels(pixels) {
  let sum = 0;
  for (let i = 0; i < pixels.length; i += 97) {
    sum = (sum + pixels[i]) >>> 0;
  }
  return sum;
}

function makeSimulation(size, steps) {
  const sim = new WasmGrayScott(size, size, 0.06, 0.062, 0.16, 0.08, 1.0);
  sim.seed_square(Math.floor(size / 2), Math.floor(size / 2), 5, 0.5, 0.25);
  if (steps > 0) {
    sim.run(steps);
  }
  return sim;
}

function measureConversion(field, pixels, frames) {
  const start = performance.now();
  for (let frame = 0; frame < frames; frame += 1) {
    fillPixels(field, pixels);
  }
  return (performance.now() - start) / frames;
}

function measureImageData(pixels, size, frames) {
  const start = performance.now();
  for (let frame = 0; frame < frames; frame += 1) {
    new ImageData(pixels, size, size);
  }
  return (performance.now() - start) / frames;
}

function measurePutImageData(context, imageData, frames) {
  const start = performance.now();
  for (let frame = 0; frame < frames; frame += 1) {
    context.putImageData(imageData, 0, 0);
  }
  return (performance.now() - start) / frames;
}

function measureOffscreenPutImageData(imageData, size, frames) {
  if (!("OffscreenCanvas" in window)) {
    return Number.NaN;
  }
  const offscreen = new OffscreenCanvas(size, size);
  const offscreenContext = offscreen.getContext("2d", { alpha: false });
  if (!offscreenContext) {
    return Number.NaN;
  }

  const start = performance.now();
  for (let frame = 0; frame < frames; frame += 1) {
    offscreenContext.putImageData(imageData, 0, 0);
  }
  return (performance.now() - start) / frames;
}

function measureBitmapTransfer(imageData, size, frames) {
  if (!("OffscreenCanvas" in window) || !bitmapContext) {
    return Number.NaN;
  }
  const offscreen = new OffscreenCanvas(size, size);
  const offscreenContext = offscreen.getContext("2d", { alpha: false });
  if (!offscreenContext || typeof offscreen.transferToImageBitmap !== "function") {
    return Number.NaN;
  }

  const start = performance.now();
  for (let frame = 0; frame < frames; frame += 1) {
    offscreenContext.putImageData(imageData, 0, 0);
    const bitmap = offscreen.transferToImageBitmap();
    bitmapContext.transferFromImageBitmap(bitmap);
  }
  return (performance.now() - start) / frames;
}

async function runBenchmark() {
  assertElement(ctx, "2D canvas context");
  runButton.disabled = true;
  writeStatus("Running...");

  try {
    if (!wasmReady) {
      await init();
      wasmReady = true;
    }

    const size = Number.parseInt(gridInput.value, 10);
    const frames = Number.parseInt(framesInput.value, 10);
    const steps = Number.parseInt(stepsInput.value, 10);
    canvas.width = size;
    canvas.height = size;
    bitmapCanvas.width = size;
    bitmapCanvas.height = size;

    const sim = makeSimulation(size, steps);
    const field = sim.u_view();
    const pixels = new Uint8ClampedArray(size * size * 4);
    fillPixels(field, pixels);
    const imageData = new ImageData(pixels, size, size);

    const conversionMs = measureConversion(field, pixels, frames);
    const imageDataMs = measureImageData(pixels, size, frames);
    const putImageDataMs = measurePutImageData(ctx, imageData, frames);
    const offscreenPutImageDataMs = measureOffscreenPutImageData(
      imageData,
      size,
      frames,
    );
    const bitmapTransferMs = measureBitmapTransfer(imageData, size, frames);

    setResults([
      ["Float32 field to RGBA buffer", conversionMs],
      ["new ImageData(pixels, width, height)", imageDataMs],
      ["2D canvas putImageData", putImageDataMs],
      ["OffscreenCanvas putImageData", offscreenPutImageDataMs],
      ["OffscreenCanvas ImageBitmap transfer", bitmapTransferMs],
    ]);

    writeStatus({
      grid: `${size}x${size}`,
      frames,
      warmup_steps: steps,
      checksum: checksumPixels(pixels),
      user_agent: navigator.userAgent,
    });
  } catch (error) {
    writeStatus(error instanceof Error ? error.stack : String(error));
    throw error;
  } finally {
    runButton.disabled = false;
  }
}

runButton.addEventListener("click", () => {
  runBenchmark().catch((error) => {
    console.error(error);
  });
});

const shouldAutorun = applyQuerySettings();

writeStatus({
  ready: true,
  offscreen_canvas: "OffscreenCanvas" in window,
  bitmap_renderer: Boolean(bitmapContext),
  user_agent: navigator.userAgent,
});

if (shouldAutorun) {
  runBenchmark().catch((error) => {
    console.error(error);
  });
}
