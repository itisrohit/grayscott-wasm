# Manual Check: Browser Render Benchmark

Use this checklist to record real browser rendering measurements for the
Gray-Scott WASM renderer.

## Setup

Build the browser WASM package:

```bash
bash tools/build_wasm_web.sh
```

Start a local static server from the repository root:

```bash
python3 -m http.server 8000
```

Open:

```text
http://localhost:8000/www/render_bench.html
```

The same page can also be exercised through local headless Chrome:

```bash
node tools/run_browser_render_bench.mjs --grid 512 --frames 300 --steps 250
```

Use the headless path for repeatable local checks. Use the interactive browser
path when the result needs to represent visible user-facing behavior.

## Browser Settings

Record the following before collecting numbers:

- Browser name and version.
- Operating system.
- Hardware, if known.
- Page URL.
- `user_agent` from the JSON status box.

Close unrelated heavy tabs before running the benchmark. Keep the browser window
visible while measuring.

## Measurement Procedure

Use these settings unless there is a reason to test something else:

- Frames: `300`
- Warmup steps: `250`

Run each grid size three times:

- `128 x 128`
- `256 x 256`
- `512 x 512`

For each run:

1. Select the grid size.
2. Set frames to `300`.
3. Set warmup steps to `250`.
4. Click `Run`.
5. Record every visible `ms/frame` value.
6. Record the JSON status block.

Use the median of the three runs as the reported value for each grid size. Do not
use the fastest run unless explicitly reporting best-case behavior.

## Headless Chrome Procedure

Keep the local server running, then run each grid three times:

```bash
node tools/run_browser_render_bench.mjs --port 9351 --grid 128 --frames 300 --steps 250
node tools/run_browser_render_bench.mjs --port 9352 --grid 128 --frames 300 --steps 250
node tools/run_browser_render_bench.mjs --port 9353 --grid 128 --frames 300 --steps 250

node tools/run_browser_render_bench.mjs --port 9354 --grid 256 --frames 300 --steps 250
node tools/run_browser_render_bench.mjs --port 9355 --grid 256 --frames 300 --steps 250
node tools/run_browser_render_bench.mjs --port 9356 --grid 256 --frames 300 --steps 250

node tools/run_browser_render_bench.mjs --port 9357 --grid 512 --frames 300 --steps 250
node tools/run_browser_render_bench.mjs --port 9358 --grid 512 --frames 300 --steps 250
node tools/run_browser_render_bench.mjs --port 9359 --grid 512 --frames 300 --steps 250
```

Use a different DevTools `--port` for each run if Chrome has not fully released
the previous port yet.

## Metrics To Record

The page reports:

- `Float32 field to RGBA buffer`
- `new ImageData(pixels, width, height)`
- `2D canvas putImageData`
- `OffscreenCanvas putImageData`
- `OffscreenCanvas ImageBitmap transfer`

If a row says `unsupported`, record it as `unsupported`. Do not treat it as zero.

## Sanity Checks

The run is valid only if:

- Both canvases show the same visible pattern after the run.
- The JSON status box shows the expected grid, frame count, and warmup steps.
- `checksum` is nonzero.
- Larger grids are not unexpectedly faster than smaller grids across all render
  metrics.
- The browser console has no errors.

If a timing is obviously unstable, rerun that grid after closing other heavy tabs.

For headless runs, the canvas visibility check is replaced by these checks:

- The JSON output reports the expected grid, frame count, and warmup steps.
- The checksum matches the known value for the grid:
  - `128x128`: `92126`
  - `256x256`: `369296`
  - `512x512`: `1475932`
- The user agent contains `HeadlessChrome`.
- Larger grids have larger field-to-RGBA conversion times.

## Headless Check Recorded On 2026-04-27

Environment:

```text
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/147.0.0.0 Safari/537.36
```

Settings:

- Frames: `300`
- Warmup steps: `250`
- Runs per grid: `3`

Raw runs:

| Grid | Run | Float32 field to RGBA | new ImageData | 2D putImageData | OffscreenCanvas putImageData | OffscreenCanvas ImageBitmap transfer | Checksum |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 128x128 | 1 | 0.064333 | 0.000333 | 0.004000 | 0.003000 | 0.008667 | 92126 |
| 128x128 | 2 | 0.049667 | 0.000333 | 0.002000 | 0.001667 | 0.007000 | 92126 |
| 128x128 | 3 | 0.052667 | 0.000333 | 0.002000 | 0.002000 | 0.006667 | 92126 |
| 256x256 | 1 | 0.191333 | 0.000333 | 0.006667 | 0.006333 | 0.016333 | 369296 |
| 256x256 | 2 | 0.192000 | 0.000667 | 0.006000 | 0.006000 | 0.016667 | 369296 |
| 256x256 | 3 | 0.191333 | 0.000333 | 0.006000 | 0.006333 | 0.016667 | 369296 |
| 512x512 | 1 | 0.781000 | 0.000333 | 0.023000 | 0.022667 | 0.123000 | 1475932 |
| 512x512 | 2 | 0.778667 | 0.000333 | 0.022667 | 0.023000 | 0.124333 | 1475932 |
| 512x512 | 3 | 0.779333 | 0.000667 | 0.022667 | 0.023667 | 0.124667 | 1475932 |

Median results:

| Grid | Float32 field to RGBA | new ImageData | 2D putImageData | OffscreenCanvas putImageData | OffscreenCanvas ImageBitmap transfer |
| --- | ---: | ---: | ---: | ---: | ---: |
| 128x128 | 0.052667 | 0.000333 | 0.002000 | 0.002000 | 0.007000 |
| 256x256 | 0.191333 | 0.000333 | 0.006000 | 0.006333 | 0.016667 |
| 512x512 | 0.779333 | 0.000333 | 0.022667 | 0.023000 | 0.124333 |

Interpretation:

- The headless path confirms that `render_bench.html` runs end-to-end without
  manual clicking.
- Checksums match the earlier interactive Chrome measurements.
- Field-to-RGBA conversion remains the dominant render-side cost.
- Headless `putImageData` timings are lower than interactive Chrome timings, so
  do not merge those medians into the manual browser table.

## Recording Template

```text
Date:
Browser:
Browser version:
OS:
Hardware:
URL:
User agent:

Settings:
- Frames: 300
- Warmup steps: 250

Grid: 128 x 128
Run 1:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
- checksum:
Run 2:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
- checksum:
Run 3:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
- checksum:
Median:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:

Grid: 256 x 256
Run 1:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
- checksum:
Run 2:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
- checksum:
Run 3:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
- checksum:
Median:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:

Grid: 512 x 512
Run 1:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
- checksum:
Run 2:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
- checksum:
Run 3:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
- checksum:
Median:
- Float32 field to RGBA buffer:
- new ImageData:
- 2D canvas putImageData:
- OffscreenCanvas putImageData:
- OffscreenCanvas ImageBitmap transfer:
```
