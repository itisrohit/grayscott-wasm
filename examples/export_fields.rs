use grayscott_wasm::{GrayScott, GrayScottParams};
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Args {
    width: usize,
    height: usize,
    steps: usize,
    radius: usize,
    feed: f32,
    kill: f32,
    diff_u: f32,
    diff_v: f32,
    dt: f32,
    output_dir: PathBuf,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            steps: 100,
            radius: 5,
            feed: 0.060,
            kill: 0.062,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
            output_dir: PathBuf::from("data/rust_fields"),
        }
    }
}

fn parse_args() -> Args {
    let mut args = Args::default();
    let mut iter = env::args().skip(1);

    while let Some(flag) = iter.next() {
        let value = iter
            .next()
            .unwrap_or_else(|| panic!("missing value for {flag}"));
        match flag.as_str() {
            "--width" => args.width = value.parse().expect("invalid --width"),
            "--height" => args.height = value.parse().expect("invalid --height"),
            "--steps" => args.steps = value.parse().expect("invalid --steps"),
            "--radius" => args.radius = value.parse().expect("invalid --radius"),
            "--feed" => args.feed = value.parse().expect("invalid --feed"),
            "--kill" => args.kill = value.parse().expect("invalid --kill"),
            "--diff-u" => args.diff_u = value.parse().expect("invalid --diff-u"),
            "--diff-v" => args.diff_v = value.parse().expect("invalid --diff-v"),
            "--dt" => args.dt = value.parse().expect("invalid --dt"),
            "--output-dir" => args.output_dir = PathBuf::from(value),
            _ => panic!("unknown argument: {flag}"),
        }
    }

    args
}

fn write_f32_raw(path: &Path, values: &[f32]) {
    let file = File::create(path).unwrap_or_else(|err| panic!("failed to create {path:?}: {err}"));
    let mut writer = BufWriter::new(file);
    for &value in values {
        writer
            .write_all(&value.to_le_bytes())
            .unwrap_or_else(|err| panic!("failed to write {path:?}: {err}"));
    }
    writer
        .flush()
        .unwrap_or_else(|err| panic!("failed to flush {path:?}: {err}"));
}

fn main() {
    let args = parse_args();
    fs::create_dir_all(&args.output_dir)
        .unwrap_or_else(|err| panic!("failed to create {:?}: {err}", args.output_dir));

    let mut sim = GrayScott::new(args.width, args.height);
    sim.seed_square(args.width / 2, args.height / 2, args.radius);
    sim.run(
        args.steps,
        GrayScottParams::new(args.feed, args.kill, args.diff_u, args.diff_v, args.dt),
    );

    let u_path = args.output_dir.join("u_f32_le.raw");
    let v_path = args.output_dir.join("v_f32_le.raw");
    let meta_path = args.output_dir.join("metadata.json");

    write_f32_raw(&u_path, sim.u());
    write_f32_raw(&v_path, sim.v());

    let metadata = format!(
        concat!(
            "{{\n",
            "  \"width\": {},\n",
            "  \"height\": {},\n",
            "  \"steps\": {},\n",
            "  \"radius\": {},\n",
            "  \"feed\": {},\n",
            "  \"kill\": {},\n",
            "  \"diff_u\": {},\n",
            "  \"diff_v\": {},\n",
            "  \"dt\": {},\n",
            "  \"dtype\": \"f32_le\",\n",
            "  \"u\": \"u_f32_le.raw\",\n",
            "  \"v\": \"v_f32_le.raw\"\n",
            "}}\n"
        ),
        args.width,
        args.height,
        args.steps,
        args.radius,
        args.feed,
        args.kill,
        args.diff_u,
        args.diff_v,
        args.dt
    );
    fs::write(&meta_path, metadata)
        .unwrap_or_else(|err| panic!("failed to write {meta_path:?}: {err}"));

    println!("{}", args.output_dir.display());
}
