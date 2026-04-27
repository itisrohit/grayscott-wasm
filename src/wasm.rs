use crate::{
    add_uniform_noise, forward_gradient_descent_backtracking, generate_target, loss_for_params,
    BacktrackingConfig, GradientDescentConfig, GrayScott, GrayScottParams, InverseTarget,
};
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmGrayScott {
    inner: GrayScott,
    params: GrayScottParams,
}

#[wasm_bindgen]
impl WasmGrayScott {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            inner: GrayScott::new(width, height),
            params: GrayScottParams::default(),
        }
    }

    pub fn width(&self) -> usize {
        self.inner.width()
    }

    pub fn height(&self) -> usize {
        self.inner.height()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn set_params(&mut self, feed: f32, kill: f32, diff_u: f32, diff_v: f32, dt: f32) {
        self.params = GrayScottParams::new(feed, kill, diff_u, diff_v, dt);
    }

    pub fn seed_square(&mut self, center_x: usize, center_y: usize, radius: usize) {
        self.inner.seed_square(center_x, center_y, radius);
    }

    pub fn step(&mut self) {
        self.inner.step(self.params);
    }

    pub fn run(&mut self, steps: usize) {
        self.inner.run(steps, self.params);
    }

    pub fn run_simd(&mut self, steps: usize) {
        self.inner.run_simd(steps, self.params);
    }

    pub fn simd_enabled(&self) -> bool {
        cfg!(all(target_arch = "wasm32", target_feature = "simd128"))
    }

    pub fn checksum(&self) -> f64 {
        self.inner
            .u()
            .iter()
            .chain(self.inner.v())
            .fold(0.0f64, |sum, &value| sum + f64::from(value))
    }

    pub fn u_ptr(&self) -> *const f32 {
        self.inner.u().as_ptr()
    }

    pub fn v_ptr(&self) -> *const f32 {
        self.inner.v().as_ptr()
    }

    pub fn u_view(&self) -> Float32Array {
        unsafe { Float32Array::view(self.inner.u()) }
    }

    pub fn v_view(&self) -> Float32Array {
        unsafe { Float32Array::view(self.inner.v()) }
    }

    pub fn u_values(&self) -> Vec<f32> {
        self.inner.u().to_vec()
    }

    pub fn v_values(&self) -> Vec<f32> {
        self.inner.v().to_vec()
    }
}

#[wasm_bindgen]
pub fn inverse_ad_line_json(
    width: usize,
    height: usize,
    steps: usize,
    radius: usize,
    target_feed: f32,
    target_kill: f32,
    initial_feed: f32,
    initial_kill: f32,
    iterations: usize,
    learning_rate: f32,
    noise_amplitude: f32,
    noise_seed: u32,
) -> String {
    let params = GrayScottParams::new(target_feed, target_kill, 0.16, 0.08, 1.0);
    let target = InverseTarget::new(width, height, steps, radius, params);
    let (clean_u, clean_v) = generate_target(target);
    let mut target_u = clean_u.clone();
    let mut target_v = clean_v.clone();
    add_uniform_noise(&mut target_u, noise_amplitude, u64::from(noise_seed));
    add_uniform_noise(
        &mut target_v,
        noise_amplitude,
        u64::from(noise_seed) ^ 0xa5a5_a5a5,
    );

    let descent = GradientDescentConfig {
        initial_feed,
        initial_kill,
        feed_min: 0.045,
        feed_max: 0.070,
        kill_min: 0.055,
        kill_max: 0.070,
        learning_rate,
        epsilon: 1.0e-4,
        iterations,
        diff_u: 0.16,
        diff_v: 0.08,
        dt: 1.0,
    };
    let config = BacktrackingConfig {
        descent,
        shrink: 0.5,
        armijo: 1.0e-4,
        min_step: 1.0e-8,
        max_backtracks: 12,
    };
    let result = forward_gradient_descent_backtracking(target, config, &target_u, &target_v);
    let first = result.steps.first().expect("optimizer produced no steps");
    let last = result.steps.last().expect("optimizer produced no steps");
    let final_params = GrayScottParams::new(last.feed, last.kill, 0.16, 0.08, 1.0);
    let clean_loss = loss_for_params(target, final_params, &clean_u, &clean_v);

    let mut step_json = String::new();
    for (index, step) in result.steps.iter().enumerate() {
        if index > 0 {
            step_json.push(',');
        }
        step_json.push_str(&format!(
            "{{\"iteration\":{},\"feed\":{:.9},\"kill\":{:.9},\"loss\":{:.9e},\"grad_feed\":{:.9e},\"grad_kill\":{:.9e}}}",
            step.iteration, step.feed, step.kill, step.loss, step.grad_feed, step.grad_kill
        ));
    }

    format!(
        "{{\"grid\":\"{}x{}\",\"steps\":{},\"radius\":{},\"target_feed\":{:.9},\"target_kill\":{:.9},\"initial_feed\":{:.9},\"initial_kill\":{:.9},\"iterations\":{},\"learning_rate\":{:.9e},\"noise_amplitude\":{:.9},\"noise_seed\":{},\"initial_loss\":{:.9e},\"final_feed\":{:.9},\"final_kill\":{:.9},\"feed_abs_error\":{:.9},\"kill_abs_error\":{:.9},\"final_loss_noisy\":{:.9e},\"final_loss_clean\":{:.9e},\"evaluated\":{},\"steps_history\":[{}]}}",
        width,
        height,
        steps,
        radius,
        target_feed,
        target_kill,
        initial_feed,
        initial_kill,
        iterations,
        learning_rate,
        noise_amplitude,
        noise_seed,
        first.loss,
        last.feed,
        last.kill,
        (last.feed - target_feed).abs(),
        (last.kill - target_kill).abs(),
        last.loss,
        clean_loss,
        result.evaluated,
        step_json
    )
}
