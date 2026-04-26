use grayscott_wasm::{GrayScott, GrayScottParams};
use std::hint::black_box;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Args {
    grids: Vec<usize>,
    steps: usize,
    trials: usize,
    warmup_steps: usize,
    radius: usize,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            grids: vec![128, 256, 512],
            steps: 500,
            trials: 5,
            warmup_steps: 25,
            radius: 5,
        }
    }
}

#[derive(Debug)]
struct BenchRow {
    grid: usize,
    steps: usize,
    trials: usize,
    median_ms_per_step: f64,
    min_ms_per_step: f64,
    max_ms_per_step: f64,
    median_steps_per_sec: f64,
    cells_per_sec: f64,
    checksum: f64,
}

fn parse_args() -> Args {
    let mut args = Args::default();
    let mut iter = std::env::args().skip(1);

    while let Some(flag) = iter.next() {
        let value = iter
            .next()
            .unwrap_or_else(|| panic!("missing value for {flag}"));
        match flag.as_str() {
            "--grids" => {
                args.grids = value
                    .split(',')
                    .map(|part| part.parse().expect("invalid --grids value"))
                    .collect();
            }
            "--steps" => args.steps = value.parse().expect("invalid --steps"),
            "--trials" => args.trials = value.parse().expect("invalid --trials"),
            "--warmup-steps" => {
                args.warmup_steps = value.parse().expect("invalid --warmup-steps");
            }
            "--radius" => args.radius = value.parse().expect("invalid --radius"),
            _ => panic!("unknown argument: {flag}"),
        }
    }

    assert!(!args.grids.is_empty(), "--grids must not be empty");
    assert!(args.steps > 0, "--steps must be non-zero");
    assert!(args.trials > 0, "--trials must be non-zero");
    args
}

fn checksum(sim: &GrayScott) -> f64 {
    sim.u()
        .iter()
        .chain(sim.v())
        .fold(0.0f64, |sum, &value| sum + f64::from(value))
}

fn seeded_sim(grid: usize, radius: usize) -> GrayScott {
    let mut sim = GrayScott::new(grid, grid);
    sim.seed_square(grid / 2, grid / 2, radius);
    sim
}

fn duration_per_step(duration: Duration, steps: usize) -> f64 {
    duration.as_secs_f64() * 1_000.0 / steps as f64
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);
    let mid = values.len() / 2;
    if values.len().is_multiple_of(2) {
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[mid]
    }
}

fn run_grid(grid: usize, args: &Args, params: GrayScottParams) -> BenchRow {
    let mut warmup = seeded_sim(grid, args.radius);
    warmup.run(args.warmup_steps, params);
    black_box(checksum(&warmup));

    let mut ms_per_step = Vec::with_capacity(args.trials);
    let mut last_checksum = 0.0;

    for _ in 0..args.trials {
        let mut sim = seeded_sim(grid, args.radius);
        let start = Instant::now();
        sim.run(args.steps, params);
        let duration = start.elapsed();
        last_checksum = checksum(&sim);
        black_box(last_checksum);
        ms_per_step.push(duration_per_step(duration, args.steps));
    }

    let min_ms_per_step = ms_per_step.iter().copied().fold(f64::INFINITY, f64::min);
    let max_ms_per_step = ms_per_step
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let median_ms_per_step = median(&mut ms_per_step);
    let median_steps_per_sec = 1_000.0 / median_ms_per_step;
    let cells_per_sec = median_steps_per_sec * (grid * grid) as f64;

    BenchRow {
        grid,
        steps: args.steps,
        trials: args.trials,
        median_ms_per_step,
        min_ms_per_step,
        max_ms_per_step,
        median_steps_per_sec,
        cells_per_sec,
        checksum: last_checksum,
    }
}

fn print_markdown(rows: &[BenchRow]) {
    println!(
        "| Grid | Steps | Trials | Median ms/step | Min ms/step | Max ms/step | Median steps/s | Cells/s | Checksum |"
    );
    println!("|---|---:|---:|---:|---:|---:|---:|---:|---:|");
    for row in rows {
        println!(
            "| {0}x{0} | {1} | {2} | {3:.6} | {4:.6} | {5:.6} | {6:.2} | {7:.3e} | {8:.6} |",
            row.grid,
            row.steps,
            row.trials,
            row.median_ms_per_step,
            row.min_ms_per_step,
            row.max_ms_per_step,
            row.median_steps_per_sec,
            row.cells_per_sec,
            row.checksum
        );
    }
}

fn main() {
    let args = parse_args();
    let params = GrayScottParams::default();
    let rows: Vec<_> = args
        .grids
        .iter()
        .map(|&grid| run_grid(grid, &args, params))
        .collect();

    print_markdown(&rows);
}
