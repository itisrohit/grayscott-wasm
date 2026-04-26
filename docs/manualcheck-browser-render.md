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
