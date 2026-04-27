use crate::{GrayScott, GrayScottParams};
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
