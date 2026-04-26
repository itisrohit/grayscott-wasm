use std::env;

use grayscott_wasm::{
    generate_target, grid_search, GrayScottParams, GridSearchConfig, InverseTarget,
};

#[derive(Debug)]
struct Args {
    width: usize,
    height: usize,
    steps: usize,
    radius: usize,
    target_feed: f32,
    target_kill: f32,
    diff_u: f32,
    diff_v: f32,
    dt: f32,
    feed_min: f32,
    feed_max: f32,
    feed_count: usize,
    kill_min: f32,
    kill_max: f32,
    kill_count: usize,
    expect_feed: Option<f32>,
    expect_kill: Option<f32>,
    tolerance: f32,
    json: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            steps: 100,
            radius: 5,
            target_feed: 0.060,
            target_kill: 0.062,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
            feed_min: 0.050,
            feed_max: 0.070,
            feed_count: 21,
            kill_min: 0.055,
            kill_max: 0.070,
            kill_count: 16,
            expect_feed: None,
            expect_kill: None,
            tolerance: 1.0e-6,
            json: false,
        }
    }
}

fn main() {
    let args = Args::parse(env::args().skip(1));
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
    let config = GridSearchConfig {
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

    let (target_u, target_v) = generate_target(target);
    let result = grid_search(target, config, &target_u, &target_v);

    if let Some(expected) = args.expect_feed {
        assert_within("feed", result.best_feed, expected, args.tolerance);
    }
    if let Some(expected) = args.expect_kill {
        assert_within("kill", result.best_kill, expected, args.tolerance);
    }

    if args.json {
        println!(
            "{{\"width\":{},\"height\":{},\"steps\":{},\"radius\":{},\"target_feed\":{:.9},\"target_kill\":{:.9},\"best_feed\":{:.9},\"best_kill\":{:.9},\"best_loss\":{:.9e},\"evaluated\":{}}}",
            args.width,
            args.height,
            args.steps,
            args.radius,
            args.target_feed,
            args.target_kill,
            result.best_feed,
            result.best_kill,
            result.best_loss,
            result.evaluated
        );
        return;
    }

    println!("| Grid | Steps | Target F | Target k | Best F | Best k | Loss | Evaluated |");
    println!("|---|---:|---:|---:|---:|---:|---:|---:|");
    println!(
        "| {}x{} | {} | {:.6} | {:.6} | {:.6} | {:.6} | {:.3e} | {} |",
        args.width,
        args.height,
        args.steps,
        args.target_feed,
        args.target_kill,
        result.best_feed,
        result.best_kill,
        result.best_loss,
        result.evaluated
    );
}

impl Args {
    fn parse<I>(mut values: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut args = Self::default();
        while let Some(flag) = values.next() {
            match flag.as_str() {
                "--width" => args.width = parse_next(&flag, &mut values),
                "--height" => args.height = parse_next(&flag, &mut values),
                "--steps" => args.steps = parse_next(&flag, &mut values),
                "--radius" => args.radius = parse_next(&flag, &mut values),
                "--target-feed" => args.target_feed = parse_next(&flag, &mut values),
                "--target-kill" => args.target_kill = parse_next(&flag, &mut values),
                "--diff-u" => args.diff_u = parse_next(&flag, &mut values),
                "--diff-v" => args.diff_v = parse_next(&flag, &mut values),
                "--dt" => args.dt = parse_next(&flag, &mut values),
                "--feed-min" => args.feed_min = parse_next(&flag, &mut values),
                "--feed-max" => args.feed_max = parse_next(&flag, &mut values),
                "--feed-count" => args.feed_count = parse_next(&flag, &mut values),
                "--kill-min" => args.kill_min = parse_next(&flag, &mut values),
                "--kill-max" => args.kill_max = parse_next(&flag, &mut values),
                "--kill-count" => args.kill_count = parse_next(&flag, &mut values),
                "--expect-feed" => args.expect_feed = Some(parse_next(&flag, &mut values)),
                "--expect-kill" => args.expect_kill = Some(parse_next(&flag, &mut values)),
                "--tolerance" => args.tolerance = parse_next(&flag, &mut values),
                "--json" => args.json = true,
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                unknown => panic!("unknown argument: {unknown}"),
            }
        }
        assert!(args.width > 0, "width must be non-zero");
        assert!(args.height > 0, "height must be non-zero");
        assert!(args.feed_count > 0, "feed-count must be non-zero");
        assert!(args.kill_count > 0, "kill-count must be non-zero");
        args
    }
}

fn parse_next<T, I>(flag: &str, values: &mut I) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    I: Iterator<Item = String>,
{
    let raw = values
        .next()
        .unwrap_or_else(|| panic!("{flag} requires a value"));
    raw.parse::<T>()
        .unwrap_or_else(|err| panic!("invalid value for {flag}: {raw} ({err})"))
}

fn assert_within(name: &str, actual: f32, expected: f32, tolerance: f32) {
    let delta = (actual - expected).abs();
    if delta > tolerance {
        eprintln!(
            "{name} mismatch: actual={actual:.9} expected={expected:.9} delta={delta:.9} tolerance={tolerance:.9}"
        );
        std::process::exit(1);
    }
}

fn print_help() {
    println!(
        "Usage: cargo run --bin inverse_grid -- [options]\n\
\n\
Options:\n\
  --width N              Grid width [default: 64]\n\
  --height N             Grid height [default: 64]\n\
  --steps N              Solver steps [default: 100]\n\
  --radius N             Seed radius [default: 5]\n\
  --target-feed F        Target feed rate [default: 0.060]\n\
  --target-kill K        Target kill rate [default: 0.062]\n\
  --feed-min F           Search feed minimum [default: 0.050]\n\
  --feed-max F           Search feed maximum [default: 0.070]\n\
  --feed-count N         Search feed samples [default: 21]\n\
  --kill-min K           Search kill minimum [default: 0.055]\n\
  --kill-max K           Search kill maximum [default: 0.070]\n\
  --kill-count N         Search kill samples [default: 16]\n\
  --expect-feed F        Fail unless recovered feed is within tolerance\n\
  --expect-kill K        Fail unless recovered kill is within tolerance\n\
  --tolerance EPS        Expectation tolerance [default: 1e-6]\n\
  --json                 Print one JSON object"
    );
}
