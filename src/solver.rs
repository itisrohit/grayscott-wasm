#[derive(Debug, Clone, Copy)]
pub struct GrayScottParams {
    pub feed: f32,
    pub kill: f32,
    pub diff_u: f32,
    pub diff_v: f32,
    pub dt: f32,
}

impl GrayScottParams {
    pub const fn new(feed: f32, kill: f32, diff_u: f32, diff_v: f32, dt: f32) -> Self {
        Self {
            feed,
            kill,
            diff_u,
            diff_v,
            dt,
        }
    }
}

impl Default for GrayScottParams {
    fn default() -> Self {
        Self {
            feed: 0.060,
            kill: 0.062,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GrayScott {
    width: usize,
    height: usize,
    u: Vec<f32>,
    v: Vec<f32>,
    next_u: Vec<f32>,
    next_v: Vec<f32>,
}

impl GrayScott {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0, "width must be non-zero");
        assert!(height > 0, "height must be non-zero");

        let len = width
            .checked_mul(height)
            .expect("grid dimensions overflow usize");

        Self {
            width,
            height,
            u: vec![1.0; len],
            v: vec![0.0; len],
            next_u: vec![1.0; len],
            next_v: vec![0.0; len],
        }
    }

    pub fn from_fields(width: usize, height: usize, u: Vec<f32>, v: Vec<f32>) -> Self {
        assert!(width > 0, "width must be non-zero");
        assert!(height > 0, "height must be non-zero");

        let len = width
            .checked_mul(height)
            .expect("grid dimensions overflow usize");
        assert_eq!(u.len(), len, "u length must equal width * height");
        assert_eq!(v.len(), len, "v length must equal width * height");

        Self {
            width,
            height,
            next_u: u.clone(),
            next_v: v.clone(),
            u,
            v,
        }
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub fn len(&self) -> usize {
        self.u.len()
    }

    pub fn is_empty(&self) -> bool {
        self.u.is_empty()
    }

    pub fn u(&self) -> &[f32] {
        &self.u
    }

    pub fn v(&self) -> &[f32] {
        &self.v
    }

    pub fn u_mut(&mut self) -> &mut [f32] {
        &mut self.u
    }

    pub fn v_mut(&mut self) -> &mut [f32] {
        &mut self.v
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width, "x out of bounds");
        assert!(y < self.height, "y out of bounds");
        self.idx(x, y)
    }

    pub fn seed_square(&mut self, center_x: usize, center_y: usize, radius: usize) {
        assert!(center_x < self.width, "center_x out of bounds");
        assert!(center_y < self.height, "center_y out of bounds");

        let min_x = center_x.saturating_sub(radius);
        let max_x = center_x.saturating_add(radius).min(self.width - 1);
        let min_y = center_y.saturating_sub(radius);
        let max_y = center_y.saturating_add(radius).min(self.height - 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let i = self.idx(x, y);
                self.u[i] = 0.50;
                self.v[i] = 0.25;
            }
        }
        self.next_u.copy_from_slice(&self.u);
        self.next_v.copy_from_slice(&self.v);
    }

    pub fn step(&mut self, params: GrayScottParams) {
        let width = self.width;
        let height = self.height;

        for y in 0..height {
            for x in 0..width {
                self.update_cell_scalar(x, y, params);
            }
        }

        core::mem::swap(&mut self.u, &mut self.next_u);
        core::mem::swap(&mut self.v, &mut self.next_v);
    }

    pub fn step_simd(&mut self, params: GrayScottParams) {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            self.step_wasm_simd(params);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.step(params);
        }
    }

    pub fn run(&mut self, steps: usize, params: GrayScottParams) {
        for _ in 0..steps {
            self.step(params);
        }
    }

    pub fn run_simd(&mut self, steps: usize, params: GrayScottParams) {
        for _ in 0..steps {
            self.step_simd(params);
        }
    }

    #[inline]
    const fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    fn update_cell_scalar(&mut self, x: usize, y: usize, params: GrayScottParams) {
        let y_up = if y == 0 { self.height - 1 } else { y - 1 };
        let y_down = if y + 1 == self.height { 0 } else { y + 1 };
        let x_left = if x == 0 { self.width - 1 } else { x - 1 };
        let x_right = if x + 1 == self.width { 0 } else { x + 1 };

        let center = y * self.width + x;
        let left = y * self.width + x_left;
        let right = y * self.width + x_right;
        let up = y_up * self.width + x;
        let down = y_down * self.width + x;

        let u = self.u[center];
        let v = self.v[center];
        let lap_u = self.u[left] + self.u[right] + self.u[up] + self.u[down] - 4.0 * u;
        let lap_v = self.v[left] + self.v[right] + self.v[up] + self.v[down] - 4.0 * v;
        let uvv = u * v * v;

        self.next_u[center] =
            u + params.dt * (params.diff_u * lap_u - uvv + params.feed * (1.0 - u));
        self.next_v[center] =
            v + params.dt * (params.diff_v * lap_v + uvv - (params.feed + params.kill) * v);
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[target_feature(enable = "simd128")]
    unsafe fn step_wasm_simd(&mut self, params: GrayScottParams) {
        use core::arch::wasm32::{
            f32x4_add, f32x4_mul, f32x4_splat, f32x4_sub, v128_load, v128_store,
        };

        let width = self.width;
        let height = self.height;
        if width < 6 || height < 3 {
            self.step(params);
            return;
        }

        for x in 0..width {
            self.update_cell_scalar(x, 0, params);
            self.update_cell_scalar(x, height - 1, params);
        }
        for y in 1..height - 1 {
            self.update_cell_scalar(0, y, params);
            self.update_cell_scalar(width - 1, y, params);
        }

        let four = f32x4_splat(4.0);
        let one = f32x4_splat(1.0);
        let dt = f32x4_splat(params.dt);
        let feed = f32x4_splat(params.feed);
        let feed_plus_kill = f32x4_splat(params.feed + params.kill);
        let diff_u = f32x4_splat(params.diff_u);
        let diff_v = f32x4_splat(params.diff_v);

        for y in 1..height - 1 {
            let row = y * width;
            let up_row = (y - 1) * width;
            let down_row = (y + 1) * width;
            let mut x = 1;

            while x + 3 < width - 1 {
                let center = row + x;
                let u = v128_load(self.u.as_ptr().add(center).cast());
                let v = v128_load(self.v.as_ptr().add(center).cast());

                let u_left = v128_load(self.u.as_ptr().add(center - 1).cast());
                let u_right = v128_load(self.u.as_ptr().add(center + 1).cast());
                let u_up = v128_load(self.u.as_ptr().add(up_row + x).cast());
                let u_down = v128_load(self.u.as_ptr().add(down_row + x).cast());
                let v_left = v128_load(self.v.as_ptr().add(center - 1).cast());
                let v_right = v128_load(self.v.as_ptr().add(center + 1).cast());
                let v_up = v128_load(self.v.as_ptr().add(up_row + x).cast());
                let v_down = v128_load(self.v.as_ptr().add(down_row + x).cast());

                let lap_u = f32x4_sub(
                    f32x4_add(f32x4_add(u_left, u_right), f32x4_add(u_up, u_down)),
                    f32x4_mul(four, u),
                );
                let lap_v = f32x4_sub(
                    f32x4_add(f32x4_add(v_left, v_right), f32x4_add(v_up, v_down)),
                    f32x4_mul(four, v),
                );
                let uvv = f32x4_mul(f32x4_mul(u, v), v);

                let next_u = f32x4_add(
                    u,
                    f32x4_mul(
                        dt,
                        f32x4_add(
                            f32x4_sub(f32x4_mul(diff_u, lap_u), uvv),
                            f32x4_mul(feed, f32x4_sub(one, u)),
                        ),
                    ),
                );
                let next_v = f32x4_add(
                    v,
                    f32x4_mul(
                        dt,
                        f32x4_sub(
                            f32x4_add(f32x4_mul(diff_v, lap_v), uvv),
                            f32x4_mul(feed_plus_kill, v),
                        ),
                    ),
                );

                v128_store(self.next_u.as_mut_ptr().add(center).cast(), next_u);
                v128_store(self.next_v.as_mut_ptr().add(center).cast(), next_v);
                x += 4;
            }

            while x < width - 1 {
                self.update_cell_scalar(x, y, params);
                x += 1;
            }
        }

        core::mem::swap(&mut self.u, &mut self.next_u);
        core::mem::swap(&mut self.v, &mut self.next_v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32, tolerance: f32) {
        let delta = (actual - expected).abs();
        assert!(
            delta <= tolerance,
            "actual={actual}, expected={expected}, delta={delta}, tolerance={tolerance}"
        );
    }

    #[test]
    fn new_grid_starts_at_steady_state() {
        let sim = GrayScott::new(4, 3);
        assert_eq!(sim.width(), 4);
        assert_eq!(sim.height(), 3);
        assert_eq!(sim.len(), 12);
        assert!(sim.u().iter().all(|&value| value == 1.0));
        assert!(sim.v().iter().all(|&value| value == 0.0));
    }

    #[test]
    fn uniform_steady_state_remains_unchanged() {
        let mut sim = GrayScott::new(16, 16);
        sim.run(100, GrayScottParams::default());

        for &value in sim.u() {
            assert_close(value, 1.0, 0.0);
        }
        for &value in sim.v() {
            assert_close(value, 0.0, 0.0);
        }
    }

    #[test]
    fn one_step_matches_hand_checked_periodic_update() {
        let u = vec![1.00, 0.90, 1.10, 0.80, 0.50, 1.20, 1.05, 0.95, 0.70];
        let v = vec![0.00, 0.10, 0.00, 0.20, 0.25, 0.05, 0.00, 0.15, 0.30];
        let mut sim = GrayScott::from_fields(3, 3, u, v);
        sim.step(GrayScottParams::new(0.060, 0.062, 0.16, 0.08, 1.0));

        // Center cell, periodic 5-point stencil:
        // lap_u = 0.9 + 0.95 + 0.8 + 1.2 - 4 * 0.5 = 1.85
        // lap_v = 0.1 + 0.15 + 0.2 + 0.05 - 4 * 0.25 = -0.5
        // uvv = 0.5 * 0.25 * 0.25 = 0.03125
        let center = sim.index(1, 1);
        assert_close(sim.u()[center], 0.79475, 1.0e-7);
        assert_close(sim.v()[center], 0.21075, 1.0e-7);
    }

    #[test]
    fn seeded_simulation_stays_finite_for_standard_regime() {
        let mut sim = GrayScott::new(64, 64);
        sim.seed_square(32, 32, 5);
        sim.run(1_000, GrayScottParams::default());

        assert!(sim.u().iter().all(|value| value.is_finite()));
        assert!(sim.v().iter().all(|value| value.is_finite()));
    }

    #[test]
    fn simd_entrypoint_matches_scalar_entrypoint_on_non_simd_targets() {
        let params = GrayScottParams::default();
        let mut scalar = GrayScott::new(32, 32);
        let mut simd = GrayScott::new(32, 32);
        scalar.seed_square(16, 16, 5);
        simd.seed_square(16, 16, 5);

        scalar.run(25, params);
        simd.run_simd(25, params);

        assert_eq!(scalar.u(), simd.u());
        assert_eq!(scalar.v(), simd.v());
    }
}
