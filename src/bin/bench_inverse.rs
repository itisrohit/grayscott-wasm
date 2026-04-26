use std::hint::black_box;
use std::time::{Duration, Instant};

use grayscott_wasm::{
    finite_difference_gradient, forward_gradient, generate_target, loss_for_params,
    GrayScottParams, InverseTarget,
};

#[derive(Debug)]
struct Args {
    width: usize,
    height: usize,
    steps: usize,
    trials: usize,
    radius: usize,
    target_feed: f32,
    target_kill: f32,
    guess_feed: f32,
    guess_kill: f32,
    diff_u: f32,
    diff_v: f32,
    dt: f32,
    epsilon: f32,
}

#[derive(Debug)]
struct BenchRow {
    name: &'static str,
    evaluated: usize,
    median_ms: f64,
    min_ms: f64,
    max_ms: f64,
    overhead_vs_primal: f64,
    checksum: f64,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            steps: 100,
            trials: 7,
            radius: 5,
            target_feed: 0.06055,
            target_kill: 0.06245,
            guess_feed: 0.060,
            guess_kill: 0.063,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
            epsilon: 1.0e-4,
        }
    }
}

fn main() {
    let args = Args::parse(std::env::args().skip(1));
    let target_params = GrayScottParams::new(
        args.target_feed,
        args.target_kill,
        args.diff_u,
        args.diff_v,
        args.dt,
    );
    let guess_params = GrayScottParams::new(
        args.guess_feed,
        args.guess_kill,
        args.diff_u,
        args.diff_v,
        args.dt,
    );
    let target = InverseTarget::new(
        args.width,
        args.height,
        args.steps,
        args.radius,
        target_params,
    );
    let (target_u, target_v) = generate_target(target);

    let primal = bench(&args, "Primal loss", 1, || {
        loss_for_params(target, guess_params, &target_u, &target_v)
    });
    let finite = bench(&args, "Finite difference gradient", 5, || {
        let gradient =
            finite_difference_gradient(target, guess_params, &target_u, &target_v, args.epsilon);
        gradient.base_loss + gradient.feed + gradient.kill
    });
    let forward = bench(&args, "Forward-mode AD gradient", 1, || {
        let gradient = forward_gradient(target, guess_params, &target_u, &target_v);
        gradient.loss + gradient.feed + gradient.kill
    });

    let primal_median = primal.median_ms;
    let rows = [
        primal.with_overhead(primal_median),
        finite.with_overhead(primal_median),
        forward.with_overhead(primal_median),
    ];
    print_markdown(&args, &rows);
}

impl Args {
    fn parse<I>(mut values: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut args = Self::default();
        while let Some(flag) = values.next() {
            let value = values
                .next()
                .unwrap_or_else(|| panic!("missing value for {flag}"));
            match flag.as_str() {
                "--width" => args.width = value.parse().expect("invalid --width"),
                "--height" => args.height = value.parse().expect("invalid --height"),
                "--steps" => args.steps = value.parse().expect("invalid --steps"),
                "--trials" => args.trials = value.parse().expect("invalid --trials"),
                "--radius" => args.radius = value.parse().expect("invalid --radius"),
                "--target-feed" => args.target_feed = value.parse().expect("invalid --target-feed"),
                "--target-kill" => args.target_kill = value.parse().expect("invalid --target-kill"),
                "--guess-feed" => args.guess_feed = value.parse().expect("invalid --guess-feed"),
                "--guess-kill" => args.guess_kill = value.parse().expect("invalid --guess-kill"),
                "--diff-u" => args.diff_u = value.parse().expect("invalid --diff-u"),
                "--diff-v" => args.diff_v = value.parse().expect("invalid --diff-v"),
                "--dt" => args.dt = value.parse().expect("invalid --dt"),
                "--epsilon" => args.epsilon = value.parse().expect("invalid --epsilon"),
                _ => panic!("unknown argument: {flag}"),
            }
        }
        assert!(args.width > 0, "--width must be non-zero");
        assert!(args.height > 0, "--height must be non-zero");
        assert!(args.steps > 0, "--steps must be non-zero");
        assert!(args.trials > 0, "--trials must be non-zero");
        assert!(args.epsilon > 0.0, "--epsilon must be positive");
        args
    }
}

impl BenchRow {
    fn with_overhead(mut self, primal_median: f64) -> Self {
        self.overhead_vs_primal = self.median_ms / primal_median;
        self
    }
}

fn bench<F>(args: &Args, name: &'static str, evaluated: usize, mut run: F) -> BenchRow
where
    F: FnMut() -> f64,
{
    black_box(run());

    let mut durations = Vec::with_capacity(args.trials);
    let mut checksum = 0.0;
    for _ in 0..args.trials {
        let start = Instant::now();
        checksum = run();
        let duration = start.elapsed();
        black_box(checksum);
        durations.push(ms(duration));
    }

    BenchRow {
        name,
        evaluated,
        median_ms: median(&mut durations),
        min_ms: durations.iter().copied().fold(f64::INFINITY, f64::min),
        max_ms: durations.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        overhead_vs_primal: 1.0,
        checksum,
    }
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
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

fn print_markdown(args: &Args, rows: &[BenchRow]) {
    println!(
        "Grid: {}x{}, steps: {}, trials: {}",
        args.width, args.height, args.steps, args.trials
    );
    println!();
    println!(
        "| Method | Solver evaluations | Median ms | Min ms | Max ms | Overhead vs primal | Checksum |"
    );
    println!("|---|---:|---:|---:|---:|---:|---:|");
    for row in rows {
        println!(
            "| {} | {} | {:.6} | {:.6} | {:.6} | {:.2}x | {:.6e} |",
            row.name,
            row.evaluated,
            row.median_ms,
            row.min_ms,
            row.max_ms,
            row.overhead_vs_primal,
            row.checksum
        );
    }
}
