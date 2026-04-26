use grayscott_wasm::{GrayScott, GrayScottParams};

fn stats(values: &[f32]) -> (f32, f32, f32) {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    let mut sum = 0.0f64;

    for &value in values {
        min = min.min(value);
        max = max.max(value);
        sum += f64::from(value);
    }

    (min, max, (sum / values.len() as f64) as f32)
}

fn main() {
    let mut sim = GrayScott::new(64, 64);
    sim.seed_square(32, 32, 5);
    sim.run(100, GrayScottParams::default());

    let (u_min, u_max, u_mean) = stats(sim.u());
    let (v_min, v_max, v_mean) = stats(sim.v());

    println!("u_min={u_min:.9} u_max={u_max:.9} u_mean={u_mean:.9}");
    println!("v_min={v_min:.9} v_max={v_max:.9} v_mean={v_mean:.9}");
}
