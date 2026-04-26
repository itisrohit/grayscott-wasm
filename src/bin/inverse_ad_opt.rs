use grayscott_wasm::{
    add_uniform_noise, forward_gradient_descent, generate_target, grid_search, loss_for_params,
    GradientDescentConfig, GrayScottParams, GridSearchConfig, InverseTarget,
};

#[derive(Debug)]
struct Args {
    width: usize,
    height: usize,
    steps: usize,
    radius: usize,
    target_feed: f32,
    target_kill: f32,
    initial_feed: f32,
    initial_kill: f32,
    learning_rate: f32,
    iterations: usize,
    noise_levels: Vec<f32>,
    seeds: Vec<u64>,
    feed_min: f32,
    feed_max: f32,
    feed_count: usize,
    kill_min: f32,
    kill_max: f32,
    kill_count: usize,
    diff_u: f32,
    diff_v: f32,
    dt: f32,
}

struct Row<'a> {
    method: &'a str,
    feed: f32,
    kill: f32,
    noisy_loss: f64,
    evaluated: usize,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            steps: 100,
            radius: 5,
            target_feed: 0.06055,
            target_kill: 0.06245,
            initial_feed: 0.060,
            initial_kill: 0.063,
            learning_rate: 1.0e-4,
            iterations: 8,
            noise_levels: vec![0.0, 0.020, 0.050, 0.100],
            seeds: vec![0x5eed, 0x600d, 0xcafe, 0xbeef],
            feed_min: 0.045,
            feed_max: 0.070,
            feed_count: 51,
            kill_min: 0.055,
            kill_max: 0.070,
            kill_count: 31,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
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
    let target = InverseTarget::new(
        args.width,
        args.height,
        args.steps,
        args.radius,
        target_params,
    );
    let grid_config = GridSearchConfig {
        feed_min: args.feed_min,
        feed_max: args.feed_max,
        feed_count: args.feed_count,
        kill_min: args.kill_min,
        kill_max: args.kill_max,
        kill_count: args.kill_count,
        diff_u: args.diff_u,
        diff_v: args.diff_v,
        dt: args.dt,
    };
    let opt_config = GradientDescentConfig {
        initial_feed: args.initial_feed,
        initial_kill: args.initial_kill,
        feed_min: args.feed_min,
        feed_max: args.feed_max,
        kill_min: args.kill_min,
        kill_max: args.kill_max,
        learning_rate: args.learning_rate,
        epsilon: 1.0e-4,
        iterations: args.iterations,
        diff_u: args.diff_u,
        diff_v: args.diff_v,
        dt: args.dt,
    };
    let (clean_u, clean_v) = generate_target(target);

    println!(
        "Grid: {}x{}, steps: {}, AD iterations: {}, learning_rate: {:.1e}",
        args.width, args.height, args.steps, args.iterations, args.learning_rate
    );
    println!();
    println!(
        "| Noise | Seed | Method | Best/Final F | Best/Final k | F abs err | k abs err | Loss vs noisy target | Loss vs clean target | Evaluated |"
    );
    println!("|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|");

    for &noise in &args.noise_levels {
        for &seed in &args.seeds {
            let mut noisy_u = clean_u.clone();
            let mut noisy_v = clean_v.clone();
            add_uniform_noise(&mut noisy_u, noise, seed);
            add_uniform_noise(&mut noisy_v, noise, seed ^ 0xa5a5_a5a5_a5a5_a5a5);

            let grid = grid_search(target, grid_config, &noisy_u, &noisy_v);
            print_row(
                &args,
                target,
                &clean_u,
                &clean_v,
                noise,
                seed,
                Row {
                    method: "grid",
                    feed: grid.best_feed,
                    kill: grid.best_kill,
                    noisy_loss: grid.best_loss,
                    evaluated: grid.evaluated,
                },
            );

            let opt = forward_gradient_descent(target, opt_config, &noisy_u, &noisy_v);
            let last = opt.steps.last().expect("optimizer produced no steps");
            print_row(
                &args,
                target,
                &clean_u,
                &clean_v,
                noise,
                seed,
                Row {
                    method: "ad-opt",
                    feed: last.feed,
                    kill: last.kill,
                    noisy_loss: last.loss,
                    evaluated: opt.evaluated,
                },
            );
        }
    }
}

fn print_row(
    args: &Args,
    target: InverseTarget,
    clean_u: &[f32],
    clean_v: &[f32],
    noise: f32,
    seed: u64,
    row: Row<'_>,
) {
    let params = GrayScottParams::new(row.feed, row.kill, args.diff_u, args.diff_v, args.dt);
    let clean_loss = loss_for_params(target, params, clean_u, clean_v);
    println!(
        "| {:.3} | {} | {} | {:.6} | {:.6} | {:.6} | {:.6} | {:.3e} | {:.3e} | {} |",
        noise,
        seed,
        row.method,
        row.feed,
        row.kill,
        (row.feed - args.target_feed).abs(),
        (row.kill - args.target_kill).abs(),
        row.noisy_loss,
        clean_loss,
        row.evaluated
    );
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
                "--radius" => args.radius = value.parse().expect("invalid --radius"),
                "--target-feed" => args.target_feed = value.parse().expect("invalid --target-feed"),
                "--target-kill" => args.target_kill = value.parse().expect("invalid --target-kill"),
                "--initial-feed" => {
                    args.initial_feed = value.parse().expect("invalid --initial-feed")
                }
                "--initial-kill" => {
                    args.initial_kill = value.parse().expect("invalid --initial-kill")
                }
                "--learning-rate" => {
                    args.learning_rate = value.parse().expect("invalid --learning-rate")
                }
                "--iterations" => args.iterations = value.parse().expect("invalid --iterations"),
                "--noise-levels" => {
                    args.noise_levels = value
                        .split(',')
                        .map(|part| part.parse().expect("invalid --noise-levels value"))
                        .collect();
                }
                "--seed" => args.seeds = vec![value.parse().expect("invalid --seed")],
                "--seeds" => {
                    args.seeds = value
                        .split(',')
                        .map(|part| part.parse().expect("invalid --seeds value"))
                        .collect();
                }
                "--feed-min" => args.feed_min = value.parse().expect("invalid --feed-min"),
                "--feed-max" => args.feed_max = value.parse().expect("invalid --feed-max"),
                "--feed-count" => args.feed_count = value.parse().expect("invalid --feed-count"),
                "--kill-min" => args.kill_min = value.parse().expect("invalid --kill-min"),
                "--kill-max" => args.kill_max = value.parse().expect("invalid --kill-max"),
                "--kill-count" => args.kill_count = value.parse().expect("invalid --kill-count"),
                "--diff-u" => args.diff_u = value.parse().expect("invalid --diff-u"),
                "--diff-v" => args.diff_v = value.parse().expect("invalid --diff-v"),
                "--dt" => args.dt = value.parse().expect("invalid --dt"),
                _ => panic!("unknown argument: {flag}"),
            }
        }
        assert!(args.width > 0, "--width must be non-zero");
        assert!(args.height > 0, "--height must be non-zero");
        assert_eq!(args.width, args.height, "only square grids are supported");
        assert!(args.steps > 0, "--steps must be non-zero");
        assert!(args.learning_rate > 0.0, "--learning-rate must be positive");
        assert!(args.noise_levels.iter().all(|&noise| noise >= 0.0));
        assert!(!args.seeds.is_empty(), "--seeds must not be empty");
        assert!(args.feed_count > 0, "--feed-count must be non-zero");
        assert!(args.kill_count > 0, "--kill-count must be non-zero");
        args
    }
}
