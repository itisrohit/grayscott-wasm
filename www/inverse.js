import init, { inverse_ad_line_json } from "../pkg-web/grayscott_wasm.js";

const runButton = document.querySelector("#run");
const historyBody = document.querySelector("#history");
const statusOutput = document.querySelector("#status");
const finalFeed = document.querySelector("#final-feed");
const finalKill = document.querySelector("#final-kill");
const cleanLoss = document.querySelector("#clean-loss");
const evaluated = document.querySelector("#evaluated");
const elapsedMs = document.querySelector("#elapsed-ms");
const msPerEval = document.querySelector("#ms-per-eval");

let wasmReady = false;

function inputNumber(id) {
  const input = document.querySelector(id);
  if (!input) {
    throw new Error(`Missing input ${id}`);
  }
  return Number(input.value);
}

function formatFixed(value) {
  return Number.isFinite(value) ? value.toFixed(6) : "-";
}

function formatExp(value) {
  return Number.isFinite(value) ? value.toExponential(3) : "-";
}

function writeStatus(value) {
  statusOutput.textContent =
    typeof value === "string" ? value : JSON.stringify(value, null, 2);
}

function applyQuerySettings() {
  const params = new URLSearchParams(window.location.search);
  const controls = new Map([
    ["grid", "#grid"],
    ["steps", "#steps"],
    ["target_feed", "#target-feed"],
    ["target_kill", "#target-kill"],
    ["initial_feed", "#initial-feed"],
    ["initial_kill", "#initial-kill"],
    ["iterations", "#iterations"],
    ["learning_rate", "#learning-rate"],
    ["noise", "#noise"],
    ["seed", "#seed"],
  ]);

  for (const [name, selector] of controls) {
    const value = params.get(name);
    const input = document.querySelector(selector);
    if (value && input) {
      input.value = value;
    }
  }

  return params.get("autorun") === "1";
}

function setSummary(result) {
  finalFeed.textContent = formatFixed(result.final_feed);
  finalKill.textContent = formatFixed(result.final_kill);
  cleanLoss.textContent = formatExp(result.final_loss_clean);
  evaluated.textContent = String(result.evaluated);
  elapsedMs.textContent = formatFixed(result.elapsed_ms);
  msPerEval.textContent = formatFixed(result.ms_per_evaluation);
}

function setHistory(steps) {
  historyBody.replaceChildren(
    ...steps.map((step) => {
      const row = document.createElement("tr");
      for (const value of [
        step.iteration,
        formatFixed(step.feed),
        formatFixed(step.kill),
        formatExp(step.loss),
        formatExp(step.grad_feed),
        formatExp(step.grad_kill),
      ]) {
        const cell = document.createElement("td");
        cell.textContent = String(value);
        row.append(cell);
      }
      return row;
    }),
  );
}

async function runInverse() {
  runButton.disabled = true;
  writeStatus("Running...");

  try {
    if (!wasmReady) {
      await init();
      wasmReady = true;
    }

    const grid = inputNumber("#grid");
    const start = performance.now();
    const result = JSON.parse(
      inverse_ad_line_json(
        grid,
        grid,
        inputNumber("#steps"),
        5,
        inputNumber("#target-feed"),
        inputNumber("#target-kill"),
        inputNumber("#initial-feed"),
        inputNumber("#initial-kill"),
        inputNumber("#iterations"),
        inputNumber("#learning-rate"),
        inputNumber("#noise"),
        inputNumber("#seed"),
      ),
    );
    const elapsed = performance.now() - start;
    result.elapsed_ms = elapsed;
    result.ms_per_iteration = elapsed / Math.max(1, result.steps_history.length);
    result.ms_per_evaluation = elapsed / Math.max(1, result.evaluated);

    setSummary(result);
    setHistory(result.steps_history);
    writeStatus({
      grid: result.grid,
      target_feed: result.target_feed,
      target_kill: result.target_kill,
      initial_loss: result.initial_loss,
      final_loss_noisy: result.final_loss_noisy,
      final_loss_clean: result.final_loss_clean,
      feed_abs_error: result.feed_abs_error,
      kill_abs_error: result.kill_abs_error,
      evaluated: result.evaluated,
      elapsed_ms: result.elapsed_ms,
      ms_per_iteration: result.ms_per_iteration,
      ms_per_evaluation: result.ms_per_evaluation,
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
  runInverse().catch((error) => {
    console.error(error);
  });
});

const shouldAutorun = applyQuerySettings();

writeStatus({
  ready: true,
  page: "browser inverse recovery",
  user_agent: navigator.userAgent,
});

if (shouldAutorun) {
  runInverse().catch((error) => {
    console.error(error);
  });
}
