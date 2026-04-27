import init, { inverse_ad_line_json } from "../pkg-web/grayscott_wasm.js";

let wasmReady = false;

async function ensureWasmReady() {
  if (!wasmReady) {
    await init();
    wasmReady = true;
  }
}

function runInverse(payload) {
  const start = performance.now();
  const result = JSON.parse(
    inverse_ad_line_json(
      payload.grid,
      payload.grid,
      payload.steps,
      payload.radius,
      payload.targetFeed,
      payload.targetKill,
      payload.initialFeed,
      payload.initialKill,
      payload.iterations,
      payload.learningRate,
      payload.noise,
      payload.seed,
    ),
  );
  const elapsed = performance.now() - start;
  result.elapsed_ms = elapsed;
  result.ms_per_iteration = elapsed / Math.max(1, result.steps_history.length);
  result.ms_per_evaluation = elapsed / Math.max(1, result.evaluated);
  return result;
}

self.addEventListener("message", (event) => {
  const { id, payload } = event.data;
  ensureWasmReady()
    .then(() => runInverse(payload))
    .then((result) => {
      self.postMessage({ id, ok: true, result });
    })
    .catch((error) => {
      self.postMessage({
        id,
        ok: false,
        error: error instanceof Error ? error.stack : String(error),
      });
    });
});
