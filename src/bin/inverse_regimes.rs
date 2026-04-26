use grayscott_wasm::{
    generate_target, grid_search, GrayScottParams, GridSearchConfig, InverseTarget,
};

#[derive(Debug, Clone, Copy)]
struct Regime {
    name: &'static str,
    feed: f32,
    kill: f32,
}

#[derive(Debug)]
struct Args {
    width: usize,
    height: usize,
    steps: usize,
    radius: usize,
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

impl Default for Args {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            steps: 100,
            radius: 5,
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

const REGIMES: &[Regime] = &[
    Regime {
        name: "default-off-grid",
        feed: 0.06055,
        kill: 0.06245,
    },
    Regime {
        name: "lower-feed",
        feed: 0.05025,
        kill: 0.06025,
    },
    Regime {
        name: "higher-feed",
        feed: 0.06725,
        kill: 0.06475,
    },
];

fn main() {
    let args = Args::parse(std::env::args().skip(1));
    println!(
        "Grid: {}x{}, steps: {}",
        args.width, args.height, args.steps
    );
    println!();
    println!(
        "| Regime | Target F | Target k | Best F | Best k | F abs err | k abs err | Loss | Evaluated |"
    );
    println!("|---|---:|---:|---:|---:|---:|---:|---:|---:|");

    for regime in REGIMES {
        let params =
            GrayScottParams::new(regime.feed, regime.kill, args.diff_u, args.diff_v, args.dt);
        let target = InverseTarget::new(args.width, args.height, args.steps, args.radius, params);
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
        println!(
            "| {} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.3e} | {} |",
            regime.name,
            regime.feed,
            regime.kill,
            result.best_feed,
            result.best_kill,
            (result.best_feed - regime.feed).abs(),
            (result.best_kill - regime.kill).abs(),
            result.best_loss,
            result.evaluated
        );
    }
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
        assert!(args.feed_count > 0, "--feed-count must be non-zero");
        assert!(args.kill_count > 0, "--kill-count must be non-zero");
        assert!(args.feed_min <= args.feed_max, "invalid feed bounds");
        assert!(args.kill_min <= args.kill_max, "invalid kill bounds");
        args
    }
}
