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
            let y_up = if y == 0 { height - 1 } else { y - 1 };
            let y_down = if y + 1 == height { 0 } else { y + 1 };

            for x in 0..width {
                let x_left = if x == 0 { width - 1 } else { x - 1 };
                let x_right = if x + 1 == width { 0 } else { x + 1 };

                let center = y * width + x;
                let left = y * width + x_left;
                let right = y * width + x_right;
                let up = y_up * width + x;
                let down = y_down * width + x;

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
        }

        core::mem::swap(&mut self.u, &mut self.next_u);
        core::mem::swap(&mut self.v, &mut self.next_v);
    }

    pub fn run(&mut self, steps: usize, params: GrayScottParams) {
        for _ in 0..steps {
            self.step(params);
        }
    }

    #[inline]
    const fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
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
}
