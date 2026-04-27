#!/usr/bin/env node

import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawn } from "node:child_process";

const chromePath =
  process.env.CHROME_PATH ??
  "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";

function parseArgs(argv) {
  const args = {
    grid: "64",
    steps: "100",
    target_feed: "0.06055",
    target_kill: "0.06245",
    initial_feed: "0.060",
    initial_kill: "0.063",
    iterations: "8",
    learning_rate: "0.001",
    noise: "0",
    seed: "24301",
    port: "9223",
    origin: "http://127.0.0.1:8000",
  };

  for (let index = 2; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg.startsWith("--")) {
      const key = arg.slice(2).replaceAll("-", "_");
      if (!(key in args)) {
        throw new Error(`Unknown argument: ${arg}`);
      }
      args[key] = argv[++index];
    } else {
      throw new Error(`Unknown argument: ${arg}`);
    }
  }

  return args;
}

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

async function fetchJson(url) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP ${response.status} for ${url}`);
  }
  return response.json();
}

async function waitForTarget(port, targetUrl) {
  const deadline = Date.now() + 15000;
  while (Date.now() < deadline) {
    try {
      const targets = await fetchJson(`http://127.0.0.1:${port}/json`);
      const target = targets.find(
        (entry) => entry.type === "page" && entry.url === targetUrl,
      );
      if (target?.webSocketDebuggerUrl) {
        return target;
      }
    } catch {
      // Chrome may still be starting.
    }
    await sleep(100);
  }
  throw new Error("Timed out waiting for Chrome DevTools target.");
}

function connectCdp(webSocketDebuggerUrl) {
  const socket = new WebSocket(webSocketDebuggerUrl);
  let nextId = 1;
  const pending = new Map();

  socket.addEventListener("message", (event) => {
    const message = JSON.parse(event.data);
    if (message.id && pending.has(message.id)) {
      const { resolve, reject } = pending.get(message.id);
      pending.delete(message.id);
      if (message.error) {
        reject(new Error(message.error.message));
      } else {
        resolve(message.result);
      }
    }
  });

  return new Promise((resolve, reject) => {
    socket.addEventListener("open", () => {
      resolve({
        send(method, params = {}) {
          const id = nextId;
          nextId += 1;
          socket.send(JSON.stringify({ id, method, params }));
          return new Promise((sendResolve, sendReject) => {
            pending.set(id, { resolve: sendResolve, reject: sendReject });
          });
        },
        close() {
          socket.close();
        },
      });
    });
    socket.addEventListener("error", reject);
  });
}

async function waitForResult(client) {
  const expression = `(() => {
    const status = document.querySelector("#status")?.textContent ?? "";
    const historyRows = document.querySelectorAll("#history tr").length;
    return JSON.stringify({ status, historyRows });
  })()`;

  const deadline = Date.now() + 30000;
  while (Date.now() < deadline) {
    const result = await client.send("Runtime.evaluate", {
      expression,
      returnByValue: true,
    });
    const payload = JSON.parse(result.result.value);
    if (payload.historyRows > 0 && payload.status.trim().startsWith("{")) {
      return {
        ...JSON.parse(payload.status),
        history_rows: payload.historyRows,
      };
    }
    await sleep(100);
  }
  throw new Error("Timed out waiting for inverse benchmark result.");
}

function buildUrl(args) {
  const params = new URLSearchParams({ autorun: "1" });
  for (const [key, value] of Object.entries(args)) {
    if (key !== "port" && key !== "origin") {
      params.set(key, value);
    }
  }
  return `${args.origin}/www/inverse.html?${params.toString()}`;
}

async function main() {
  const args = parseArgs(process.argv);
  const targetUrl = buildUrl(args);
  const userDataDir = mkdtempSync(join(tmpdir(), "grayscott-chrome-"));
  const chrome = spawn(chromePath, [
    "--headless=new",
    "--disable-gpu",
    `--remote-debugging-port=${args.port}`,
    `--user-data-dir=${userDataDir}`,
    targetUrl,
  ]);

  try {
    const target = await waitForTarget(args.port, targetUrl);
    const client = await connectCdp(target.webSocketDebuggerUrl);
    const result = await waitForResult(client);
    client.close();
    console.log(JSON.stringify(result, null, 2));
  } finally {
    chrome.kill();
    await Promise.race([
      new Promise((resolve) => {
        chrome.once("close", resolve);
      }),
      sleep(2000),
    ]);
    try {
      rmSync(userDataDir, { recursive: true, force: true });
    } catch {
      // Chrome can keep profile files briefly after exit; /tmp cleanup can reap it.
    }
  }
}

main().catch((error) => {
  console.error(error instanceof Error ? error.stack : String(error));
  process.exit(1);
});
