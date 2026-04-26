use std::env;

use grayscott_wasm::{
    generate_target, gradient_descent, GradientDescentConfig, GrayScottParams, InverseTarget,
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
    feed_min: f32,
    feed_max: f32,
    kill_min: f32,
    kill_max: f32,
    learning_rate: f32,
    epsilon: f32,
    iterations: usize,
    diff_u: f32,
    diff_v: f32,
    dt: f32,
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
            initial_feed: 0.060,
            initial_kill: 0.063,
            feed_min: 0.050,
            feed_max: 0.070,
            kill_min: 0.055,
            kill_max: 0.070,
            learning_rate: 1.0e-4,
            epsilon: 1.0e-4,
            iterations: 8,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
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
    let config = GradientDescentConfig {
        initial_feed: args.initial_feed,
        initial_kill: args.initial_kill,
        feed_min: args.feed_min,
        feed_max: args.feed_max,
        kill_min: args.kill_min,
        kill_max: args.kill_max,
        learning_rate: args.learning_rate,
        epsilon: args.epsilon,
        iterations: args.iterations,
        diff_u: args.diff_u,
        diff_v: args.diff_v,
        dt: args.dt,
    };
    let (target_u, target_v) = generate_target(target);
    let result = gradient_descent(target, config, &target_u, &target_v);
    let first = result.steps.first().expect("optimizer produced no steps");
    let last = result.steps.last().expect("optimizer produced no steps");
    let feed_abs_error = (last.feed - args.target_feed).abs();
    let kill_abs_error = (last.kill - args.target_kill).abs();

    if args.json {
        print_json(&args, &result, feed_abs_error, kill_abs_error);
        return;
    }

    println!("| Iter | F | k | Loss | dLoss/dF | dLoss/dk |");
    println!("|---:|---:|---:|---:|---:|---:|");
    for step in &result.steps {
        println!(
            "| {} | {:.6} | {:.6} | {:.3e} | {:.3e} | {:.3e} |",
            step.iteration, step.feed, step.kill, step.loss, step.grad_feed, step.grad_kill
        );
    }
    println!();
    println!("| Target F | Target k | Initial loss | Final F | Final k | F abs err | k abs err | Final loss | Evaluated |");
    println!("|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    println!(
        "| {:.6} | {:.6} | {:.3e} | {:.6} | {:.6} | {:.6} | {:.6} | {:.3e} | {} |",
        args.target_feed,
        args.target_kill,
        first.loss,
        last.feed,
        last.kill,
        feed_abs_error,
        kill_abs_error,
        last.loss,
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
                "--initial-feed" => args.initial_feed = parse_next(&flag, &mut values),
                "--initial-kill" => args.initial_kill = parse_next(&flag, &mut values),
                "--feed-min" => args.feed_min = parse_next(&flag, &mut values),
                "--feed-max" => args.feed_max = parse_next(&flag, &mut values),
                "--kill-min" => args.kill_min = parse_next(&flag, &mut values),
                "--kill-max" => args.kill_max = parse_next(&flag, &mut values),
                "--learning-rate" => args.learning_rate = parse_next(&flag, &mut values),
                "--epsilon" => args.epsilon = parse_next(&flag, &mut values),
                "--iterations" => args.iterations = parse_next(&flag, &mut values),
                "--diff-u" => args.diff_u = parse_next(&flag, &mut values),
                "--diff-v" => args.diff_v = parse_next(&flag, &mut values),
                "--dt" => args.dt = parse_next(&flag, &mut values),
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
        assert!(args.feed_min <= args.feed_max, "invalid feed bounds");
        assert!(args.kill_min <= args.kill_max, "invalid kill bounds");
        assert!(args.learning_rate > 0.0, "learning-rate must be positive");
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

fn print_json(
    args: &Args,
    result: &grayscott_wasm::GradientDescentResult,
    feed_abs_error: f32,
    kill_abs_error: f32,
) {
    let first = result.steps.first().expect("optimizer produced no steps");
    let last = result.steps.last().expect("optimizer produced no steps");
    print!(
        "{{\"width\":{},\"height\":{},\"steps\":{},\"target_feed\":{:.9},\"target_kill\":{:.9},\"initial_loss\":{:.9e},\"final_feed\":{:.9},\"final_kill\":{:.9},\"feed_abs_error\":{:.9},\"kill_abs_error\":{:.9},\"final_loss\":{:.9e},\"evaluated\":{},\"history\":[",
        args.width,
        args.height,
        args.steps,
        args.target_feed,
        args.target_kill,
        first.loss,
        last.feed,
        last.kill,
        feed_abs_error,
        kill_abs_error,
        last.loss,
        result.evaluated
    );
    for (i, step) in result.steps.iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        print!(
            "{{\"iteration\":{},\"feed\":{:.9},\"kill\":{:.9},\"loss\":{:.9e},\"grad_feed\":{:.9e},\"grad_kill\":{:.9e}}}",
            step.iteration, step.feed, step.kill, step.loss, step.grad_feed, step.grad_kill
        );
    }
    println!("]}}");
}

fn print_help() {
    println!(
        "Usage: cargo run --bin inverse_opt -- [options]\n\
\n\
Options:\n\
  --width N              Grid width [default: 64]\n\
  --height N             Grid height [default: 64]\n\
  --steps N              Solver steps [default: 100]\n\
  --radius N             Seed radius [default: 5]\n\
  --target-feed F        Target feed rate [default: 0.06055]\n\
  --target-kill K        Target kill rate [default: 0.06245]\n\
  --initial-feed F       Initial feed guess [default: 0.060]\n\
  --initial-kill K       Initial kill guess [default: 0.063]\n\
  --learning-rate LR     Gradient descent learning rate [default: 1e-4]\n\
  --epsilon EPS          Central-difference epsilon [default: 1e-4]\n\
  --iterations N         Optimization iterations [default: 8]\n\
  --json                 Print one JSON object"
    );
}
