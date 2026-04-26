use std::env;

use grayscott_wasm::{
    finite_difference_gradient, forward_gradient, generate_target, GrayScottParams, InverseTarget,
};

#[derive(Debug)]
struct Args {
    width: usize,
    height: usize,
    steps: usize,
    radius: usize,
    target_feed: f32,
    target_kill: f32,
    guess_feed: f32,
    guess_kill: f32,
    diff_u: f32,
    diff_v: f32,
    dt: f32,
    epsilon: f32,
    json: bool,
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
            guess_feed: 0.060,
            guess_kill: 0.063,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
            epsilon: 1.0e-4,
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
    let finite =
        finite_difference_gradient(target, guess_params, &target_u, &target_v, args.epsilon);
    let forward = forward_gradient(target, guess_params, &target_u, &target_v);
    let feed_abs_delta = (forward.feed - finite.feed).abs();
    let kill_abs_delta = (forward.kill - finite.kill).abs();
    let feed_rel_delta = relative_delta(forward.feed, finite.feed);
    let kill_rel_delta = relative_delta(forward.kill, finite.kill);

    if args.json {
        println!(
            "{{\"width\":{},\"height\":{},\"steps\":{},\"target_feed\":{:.9},\"target_kill\":{:.9},\"guess_feed\":{:.9},\"guess_kill\":{:.9},\"loss\":{:.9e},\"ad_grad_feed\":{:.9e},\"ad_grad_kill\":{:.9e},\"fd_grad_feed\":{:.9e},\"fd_grad_kill\":{:.9e},\"feed_abs_delta\":{:.9e},\"kill_abs_delta\":{:.9e},\"feed_rel_delta\":{:.9e},\"kill_rel_delta\":{:.9e},\"ad_evaluated\":{},\"fd_evaluated\":{}}}",
            args.width,
            args.height,
            args.steps,
            args.target_feed,
            args.target_kill,
            args.guess_feed,
            args.guess_kill,
            forward.loss,
            forward.feed,
            forward.kill,
            finite.feed,
            finite.kill,
            feed_abs_delta,
            kill_abs_delta,
            feed_rel_delta,
            kill_rel_delta,
            forward.evaluated,
            finite.evaluated
        );
        return;
    }

    println!("| Grid | Steps | Target F | Target k | Guess F | Guess k | Loss | AD dLoss/dF | FD dLoss/dF | F rel delta | AD dLoss/dk | FD dLoss/dk | k rel delta |");
    println!("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    println!(
        "| {}x{} | {} | {:.6} | {:.6} | {:.6} | {:.6} | {:.3e} | {:.3e} | {:.3e} | {:.3e} | {:.3e} | {:.3e} | {:.3e} |",
        args.width,
        args.height,
        args.steps,
        args.target_feed,
        args.target_kill,
        args.guess_feed,
        args.guess_kill,
        forward.loss,
        forward.feed,
        finite.feed,
        feed_rel_delta,
        forward.kill,
        finite.kill,
        kill_rel_delta
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
                "--guess-feed" => args.guess_feed = parse_next(&flag, &mut values),
                "--guess-kill" => args.guess_kill = parse_next(&flag, &mut values),
                "--diff-u" => args.diff_u = parse_next(&flag, &mut values),
                "--diff-v" => args.diff_v = parse_next(&flag, &mut values),
                "--dt" => args.dt = parse_next(&flag, &mut values),
                "--epsilon" => args.epsilon = parse_next(&flag, &mut values),
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
        assert!(args.epsilon > 0.0, "epsilon must be positive");
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

fn relative_delta(actual: f64, expected: f64) -> f64 {
    (actual - expected).abs() / expected.abs().max(1.0e-12)
}

fn print_help() {
    println!(
        "Usage: cargo run --bin inverse_ad -- [options]\n\
\n\
Options:\n\
  --width N              Grid width [default: 64]\n\
  --height N             Grid height [default: 64]\n\
  --steps N              Solver steps [default: 100]\n\
  --radius N             Seed radius [default: 5]\n\
  --target-feed F        Target feed rate [default: 0.06055]\n\
  --target-kill K        Target kill rate [default: 0.06245]\n\
  --guess-feed F         Gradient evaluation feed [default: 0.060]\n\
  --guess-kill K         Gradient evaluation kill [default: 0.063]\n\
  --epsilon EPS          Finite-difference epsilon [default: 1e-4]\n\
  --json                 Print one JSON object"
    );
}
