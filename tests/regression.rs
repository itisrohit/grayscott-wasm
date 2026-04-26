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

fn assert_close(actual: f32, expected: f32, tolerance: f32) {
    let delta = (actual - expected).abs();
    assert!(
        delta <= tolerance,
        "actual={actual}, expected={expected}, delta={delta}, tolerance={tolerance}"
    );
}

#[test]
fn standard_seeded_64x64_100_step_regression() {
    let mut sim = GrayScott::new(64, 64);
    sim.seed_square(32, 32, 5);
    sim.run(100, GrayScottParams::default());

    let (u_min, u_max, u_mean) = stats(sim.u());
    let (v_min, v_max, v_mean) = stats(sim.v());

    assert_close(u_min, 0.306_591_4, 1.0e-6);
    assert_close(u_max, 1.0, 1.0e-6);
    assert_close(u_mean, 0.980_694_6, 1.0e-6);
    assert_close(v_min, 0.0, 1.0e-6);
    assert_close(v_max, 0.420_273_54, 1.0e-6);
    assert_close(v_mean, 0.009767476, 1.0e-6);
}
