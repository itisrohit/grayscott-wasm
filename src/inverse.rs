use crate::{GrayScott, GrayScottParams};

#[derive(Debug, Clone, Copy)]
pub struct InverseTarget {
    pub width: usize,
    pub height: usize,
    pub steps: usize,
    pub radius: usize,
    pub params: GrayScottParams,
}

#[derive(Debug, Clone, Copy)]
pub struct GridSearchConfig {
    pub feed_min: f32,
    pub feed_max: f32,
    pub feed_count: usize,
    pub kill_min: f32,
    pub kill_max: f32,
    pub kill_count: usize,
    pub diff_u: f32,
    pub diff_v: f32,
    pub dt: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct GridSearchResult {
    pub best_feed: f32,
    pub best_kill: f32,
    pub best_loss: f64,
    pub evaluated: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct FiniteDifferenceGradient {
    pub feed: f64,
    pub kill: f64,
    pub base_loss: f64,
    pub feed_plus_loss: f64,
    pub feed_minus_loss: f64,
    pub kill_plus_loss: f64,
    pub kill_minus_loss: f64,
    pub epsilon: f32,
    pub evaluated: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct ForwardGradient {
    pub feed: f64,
    pub kill: f64,
    pub loss: f64,
    pub evaluated: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct GradientDescentConfig {
    pub initial_feed: f32,
    pub initial_kill: f32,
    pub feed_min: f32,
    pub feed_max: f32,
    pub kill_min: f32,
    pub kill_max: f32,
    pub learning_rate: f32,
    pub epsilon: f32,
    pub iterations: usize,
    pub diff_u: f32,
    pub diff_v: f32,
    pub dt: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct GradientDescentStep {
    pub iteration: usize,
    pub feed: f32,
    pub kill: f32,
    pub loss: f64,
    pub grad_feed: f64,
    pub grad_kill: f64,
}

#[derive(Debug, Clone)]
pub struct GradientDescentResult {
    pub steps: Vec<GradientDescentStep>,
    pub evaluated: usize,
}

impl InverseTarget {
    pub const fn new(
        width: usize,
        height: usize,
        steps: usize,
        radius: usize,
        params: GrayScottParams,
    ) -> Self {
        Self {
            width,
            height,
            steps,
            radius,
            params,
        }
    }
}

impl GridSearchConfig {
    pub const fn from_target(target: InverseTarget) -> Self {
        Self {
            feed_min: 0.050,
            feed_max: 0.070,
            feed_count: 21,
            kill_min: 0.055,
            kill_max: 0.070,
            kill_count: 16,
            diff_u: target.params.diff_u,
            diff_v: target.params.diff_v,
            dt: target.params.dt,
        }
    }
}

pub fn generate_target(target: InverseTarget) -> (Vec<f32>, Vec<f32>) {
    let mut sim = seeded_sim(target.width, target.height, target.radius);
    sim.run(target.steps, target.params);
    (sim.u().to_vec(), sim.v().to_vec())
}

pub fn add_uniform_noise(values: &mut [f32], amplitude: f32, seed: u64) {
    assert!(amplitude >= 0.0, "amplitude must be non-negative");
    if amplitude == 0.0 {
        return;
    }

    let mut rng = SplitMix64::new(seed);
    for value in values {
        let noise = (rng.next_f32() * 2.0 - 1.0) * amplitude;
        *value = (*value + noise).clamp(0.0, 1.0);
    }
}

pub fn field_mse(actual_u: &[f32], actual_v: &[f32], target_u: &[f32], target_v: &[f32]) -> f64 {
    assert_eq!(
        actual_u.len(),
        target_u.len(),
        "u fields must have equal length"
    );
    assert_eq!(
        actual_v.len(),
        target_v.len(),
        "v fields must have equal length"
    );
    assert_eq!(
        actual_u.len(),
        actual_v.len(),
        "u and v fields must have equal length"
    );

    let mut sum = 0.0_f64;
    for ((&u, &target_u), (&v, &target_v)) in actual_u
        .iter()
        .zip(target_u)
        .zip(actual_v.iter().zip(target_v))
    {
        let du = f64::from(u - target_u);
        let dv = f64::from(v - target_v);
        sum += du * du + dv * dv;
    }
    sum / (2 * actual_u.len()) as f64
}

pub fn loss_for_params(
    target: InverseTarget,
    params: GrayScottParams,
    target_u: &[f32],
    target_v: &[f32],
) -> f64 {
    let mut sim = seeded_sim(target.width, target.height, target.radius);
    sim.run(target.steps, params);
    field_mse(sim.u(), sim.v(), target_u, target_v)
}

pub fn finite_difference_gradient(
    target: InverseTarget,
    base_params: GrayScottParams,
    target_u: &[f32],
    target_v: &[f32],
    epsilon: f32,
) -> FiniteDifferenceGradient {
    assert!(epsilon > 0.0, "epsilon must be positive");

    let base_loss = loss_for_params(target, base_params, target_u, target_v);
    let feed_plus_loss = loss_for_params(
        target,
        GrayScottParams::new(
            base_params.feed + epsilon,
            base_params.kill,
            base_params.diff_u,
            base_params.diff_v,
            base_params.dt,
        ),
        target_u,
        target_v,
    );
    let feed_minus_loss = loss_for_params(
        target,
        GrayScottParams::new(
            base_params.feed - epsilon,
            base_params.kill,
            base_params.diff_u,
            base_params.diff_v,
            base_params.dt,
        ),
        target_u,
        target_v,
    );
    let kill_plus_loss = loss_for_params(
        target,
        GrayScottParams::new(
            base_params.feed,
            base_params.kill + epsilon,
            base_params.diff_u,
            base_params.diff_v,
            base_params.dt,
        ),
        target_u,
        target_v,
    );
    let kill_minus_loss = loss_for_params(
        target,
        GrayScottParams::new(
            base_params.feed,
            base_params.kill - epsilon,
            base_params.diff_u,
            base_params.diff_v,
            base_params.dt,
        ),
        target_u,
        target_v,
    );
    let denominator = f64::from(2.0 * epsilon);

    FiniteDifferenceGradient {
        feed: (feed_plus_loss - feed_minus_loss) / denominator,
        kill: (kill_plus_loss - kill_minus_loss) / denominator,
        base_loss,
        feed_plus_loss,
        feed_minus_loss,
        kill_plus_loss,
        kill_minus_loss,
        epsilon,
        evaluated: 5,
    }
}

pub fn forward_gradient(
    target: InverseTarget,
    base_params: GrayScottParams,
    target_u: &[f32],
    target_v: &[f32],
) -> ForwardGradient {
    let len = target
        .width
        .checked_mul(target.height)
        .expect("grid dimensions overflow usize");
    assert_eq!(
        target_u.len(),
        len,
        "target_u length must equal width * height"
    );
    assert_eq!(
        target_v.len(),
        len,
        "target_v length must equal width * height"
    );

    let mut u = vec![Dual2::constant(1.0); len];
    let mut v = vec![Dual2::constant(0.0); len];
    let mut next_u = u.clone();
    let mut next_v = v.clone();
    seed_dual_square(&mut u, &mut v, target.width, target.height, target.radius);
    next_u.copy_from_slice(&u);
    next_v.copy_from_slice(&v);

    let params = DualParams {
        feed: Dual2::variable_feed(base_params.feed),
        kill: Dual2::variable_kill(base_params.kill),
        diff_u: Dual2::constant(base_params.diff_u),
        diff_v: Dual2::constant(base_params.diff_v),
        dt: Dual2::constant(base_params.dt),
    };

    for _ in 0..target.steps {
        step_dual(
            target.width,
            target.height,
            &u,
            &v,
            &mut next_u,
            &mut next_v,
            params,
        );
        core::mem::swap(&mut u, &mut next_u);
        core::mem::swap(&mut v, &mut next_v);
    }

    let mut loss = 0.0_f64;
    let mut grad_feed = 0.0_f64;
    let mut grad_kill = 0.0_f64;
    let denom = (2 * len) as f64;
    for ((cell_u, &target_u), (cell_v, &target_v)) in
        u.iter().zip(target_u).zip(v.iter().zip(target_v))
    {
        let du = f64::from(cell_u.value - target_u);
        let dv = f64::from(cell_v.value - target_v);
        loss += du * du + dv * dv;
        grad_feed += 2.0 * du * f64::from(cell_u.d_feed) + 2.0 * dv * f64::from(cell_v.d_feed);
        grad_kill += 2.0 * du * f64::from(cell_u.d_kill) + 2.0 * dv * f64::from(cell_v.d_kill);
    }

    ForwardGradient {
        feed: grad_feed / denom,
        kill: grad_kill / denom,
        loss: loss / denom,
        evaluated: 1,
    }
}

pub fn gradient_descent(
    target: InverseTarget,
    config: GradientDescentConfig,
    target_u: &[f32],
    target_v: &[f32],
) -> GradientDescentResult {
    assert!(config.feed_min <= config.feed_max, "invalid feed bounds");
    assert!(config.kill_min <= config.kill_max, "invalid kill bounds");
    assert!(config.learning_rate > 0.0, "learning_rate must be positive");
    assert!(config.epsilon > 0.0, "epsilon must be positive");

    let mut feed = config.initial_feed.clamp(config.feed_min, config.feed_max);
    let mut kill = config.initial_kill.clamp(config.kill_min, config.kill_max);
    let mut steps = Vec::with_capacity(config.iterations + 1);
    let mut evaluated = 0;

    for iteration in 0..=config.iterations {
        let params = GrayScottParams::new(feed, kill, config.diff_u, config.diff_v, config.dt);
        let gradient =
            finite_difference_gradient(target, params, target_u, target_v, config.epsilon);
        evaluated += gradient.evaluated;
        steps.push(GradientDescentStep {
            iteration,
            feed,
            kill,
            loss: gradient.base_loss,
            grad_feed: gradient.feed,
            grad_kill: gradient.kill,
        });

        if iteration == config.iterations {
            break;
        }

        feed = (feed - config.learning_rate * gradient.feed as f32)
            .clamp(config.feed_min, config.feed_max);
        kill = (kill - config.learning_rate * gradient.kill as f32)
            .clamp(config.kill_min, config.kill_max);
    }

    GradientDescentResult { steps, evaluated }
}

pub fn grid_search(
    target: InverseTarget,
    config: GridSearchConfig,
    target_u: &[f32],
    target_v: &[f32],
) -> GridSearchResult {
    assert!(config.feed_count > 0, "feed_count must be non-zero");
    assert!(config.kill_count > 0, "kill_count must be non-zero");

    let mut best = GridSearchResult {
        best_feed: f32::NAN,
        best_kill: f32::NAN,
        best_loss: f64::INFINITY,
        evaluated: 0,
    };

    for feed in linspace(config.feed_min, config.feed_max, config.feed_count) {
        for kill in linspace(config.kill_min, config.kill_max, config.kill_count) {
            let params = GrayScottParams::new(feed, kill, config.diff_u, config.diff_v, config.dt);
            let mut sim = seeded_sim(target.width, target.height, target.radius);
            sim.run(target.steps, params);
            let loss = field_mse(sim.u(), sim.v(), target_u, target_v);
            best.evaluated += 1;

            if loss < best.best_loss {
                best.best_feed = feed;
                best.best_kill = kill;
                best.best_loss = loss;
            }
        }
    }

    best
}

pub fn linspace(min: f32, max: f32, count: usize) -> impl Iterator<Item = f32> {
    assert!(count > 0, "count must be non-zero");
    (0..count).map(move |i| {
        if count == 1 {
            min
        } else {
            let t = i as f32 / (count - 1) as f32;
            min + t * (max - min)
        }
    })
}

fn seeded_sim(width: usize, height: usize, radius: usize) -> GrayScott {
    let mut sim = GrayScott::new(width, height);
    sim.seed_square(width / 2, height / 2, radius);
    sim
}

#[derive(Debug, Clone, Copy)]
struct DualParams {
    feed: Dual2,
    kill: Dual2,
    diff_u: Dual2,
    diff_v: Dual2,
    dt: Dual2,
}

#[derive(Debug, Clone, Copy)]
struct Dual2 {
    value: f32,
    d_feed: f32,
    d_kill: f32,
}

impl Dual2 {
    const fn constant(value: f32) -> Self {
        Self {
            value,
            d_feed: 0.0,
            d_kill: 0.0,
        }
    }

    const fn variable_feed(value: f32) -> Self {
        Self {
            value,
            d_feed: 1.0,
            d_kill: 0.0,
        }
    }

    const fn variable_kill(value: f32) -> Self {
        Self {
            value,
            d_feed: 0.0,
            d_kill: 1.0,
        }
    }
}

impl core::ops::Add for Dual2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            d_feed: self.d_feed + rhs.d_feed,
            d_kill: self.d_kill + rhs.d_kill,
        }
    }
}

impl core::ops::Sub for Dual2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
            d_feed: self.d_feed - rhs.d_feed,
            d_kill: self.d_kill - rhs.d_kill,
        }
    }
}

impl core::ops::Mul for Dual2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
            d_feed: self.d_feed * rhs.value + self.value * rhs.d_feed,
            d_kill: self.d_kill * rhs.value + self.value * rhs.d_kill,
        }
    }
}

fn seed_dual_square(u: &mut [Dual2], v: &mut [Dual2], width: usize, height: usize, radius: usize) {
    let center_x = width / 2;
    let center_y = height / 2;
    let min_x = center_x.saturating_sub(radius);
    let max_x = center_x.saturating_add(radius).min(width - 1);
    let min_y = center_y.saturating_sub(radius);
    let max_y = center_y.saturating_add(radius).min(height - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let i = y * width + x;
            u[i] = Dual2::constant(0.50);
            v[i] = Dual2::constant(0.25);
        }
    }
}

fn step_dual(
    width: usize,
    height: usize,
    u: &[Dual2],
    v: &[Dual2],
    next_u: &mut [Dual2],
    next_v: &mut [Dual2],
    params: DualParams,
) {
    let four = Dual2::constant(4.0);
    let one = Dual2::constant(1.0);

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

            let u_center = u[center];
            let v_center = v[center];
            let lap_u = u[left] + u[right] + u[up] + u[down] - four * u_center;
            let lap_v = v[left] + v[right] + v[up] + v[down] - four * v_center;
            let uvv = u_center * v_center * v_center;

            next_u[center] = u_center
                + params.dt * (params.diff_u * lap_u - uvv + params.feed * (one - u_center));
            next_v[center] = v_center
                + params.dt
                    * (params.diff_v * lap_v + uvv - (params.feed + params.kill) * v_center);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn next_f32(&mut self) -> f32 {
        let value = self.next_u64() >> 40;
        value as f32 / 16_777_216.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_mse_is_zero_for_identical_fields() {
        let u = [1.0, 0.5, 0.25];
        let v = [0.0, 0.25, 0.5];
        assert_eq!(field_mse(&u, &v, &u, &v), 0.0);
    }

    #[test]
    fn add_uniform_noise_is_deterministic_and_clamped() {
        let mut first = [0.0, 0.5, 1.0];
        let mut second = [0.0, 0.5, 1.0];

        add_uniform_noise(&mut first, 0.1, 42);
        add_uniform_noise(&mut second, 0.1, 42);

        assert_eq!(first, second);
        assert!(first.iter().all(|value| (0.0..=1.0).contains(value)));
        assert_ne!(first, [0.0, 0.5, 1.0]);
    }

    #[test]
    fn loss_for_params_is_zero_at_target_parameters() {
        let params = GrayScottParams::new(0.060, 0.062, 0.16, 0.08, 1.0);
        let target = InverseTarget::new(32, 32, 50, 5, params);
        let (target_u, target_v) = generate_target(target);

        let loss = loss_for_params(target, params, &target_u, &target_v);

        assert_eq!(loss, 0.0);
    }

    #[test]
    fn finite_difference_gradient_is_finite_for_off_target_guess() {
        let target_params = GrayScottParams::new(0.06055, 0.06245, 0.16, 0.08, 1.0);
        let target = InverseTarget::new(32, 32, 100, 5, target_params);
        let (target_u, target_v) = generate_target(target);
        let guess = GrayScottParams::new(0.060, 0.063, 0.16, 0.08, 1.0);

        let gradient = finite_difference_gradient(target, guess, &target_u, &target_v, 1.0e-4);

        assert!(gradient.base_loss > 0.0);
        assert!(gradient.feed.is_finite());
        assert!(gradient.kill.is_finite());
        assert!(gradient.feed.abs() > 0.0);
        assert!(gradient.kill.abs() > 0.0);
        assert_eq!(gradient.evaluated, 5);
    }

    #[test]
    fn forward_gradient_matches_finite_difference_baseline() {
        let target_params = GrayScottParams::new(0.06055, 0.06245, 0.16, 0.08, 1.0);
        let target = InverseTarget::new(32, 32, 100, 5, target_params);
        let (target_u, target_v) = generate_target(target);
        let guess = GrayScottParams::new(0.060, 0.063, 0.16, 0.08, 1.0);

        let finite = finite_difference_gradient(target, guess, &target_u, &target_v, 1.0e-4);
        let forward = forward_gradient(target, guess, &target_u, &target_v);

        assert!((forward.loss - finite.base_loss).abs() <= 1.0e-10);
        assert_relative_close(forward.feed, finite.feed, 0.025);
        assert_relative_close(forward.kill, finite.kill, 0.025);
        assert_eq!(forward.evaluated, 1);
    }

    fn assert_relative_close(actual: f64, expected: f64, tolerance: f64) {
        let scale = expected.abs().max(1.0e-12);
        let relative = (actual - expected).abs() / scale;
        assert!(
            relative <= tolerance,
            "actual={actual}, expected={expected}, relative={relative}, tolerance={tolerance}"
        );
    }

    #[test]
    fn gradient_descent_reduces_loss_from_off_target_guess() {
        let target_params = GrayScottParams::new(0.06055, 0.06245, 0.16, 0.08, 1.0);
        let target = InverseTarget::new(32, 32, 100, 5, target_params);
        let (target_u, target_v) = generate_target(target);
        let config = GradientDescentConfig {
            initial_feed: 0.060,
            initial_kill: 0.063,
            feed_min: 0.050,
            feed_max: 0.070,
            kill_min: 0.055,
            kill_max: 0.070,
            learning_rate: 1.0e-4,
            epsilon: 1.0e-4,
            iterations: 3,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
        };

        let result = gradient_descent(target, config, &target_u, &target_v);
        let first = result.steps.first().expect("missing first step");
        let last = result.steps.last().expect("missing last step");

        assert_eq!(result.steps.len(), 4);
        assert_eq!(result.evaluated, 20);
        assert!(last.loss < first.loss);
    }

    #[test]
    fn linspace_includes_endpoints() {
        let values = linspace(0.05, 0.07, 3).collect::<Vec<_>>();
        assert_eq!(values, vec![0.05, 0.060000002, 0.07]);
    }

    #[test]
    fn grid_search_recovers_exact_parameters_when_grid_contains_target() {
        let params = GrayScottParams::new(0.060, 0.062, 0.16, 0.08, 1.0);
        let target = InverseTarget::new(32, 32, 50, 5, params);
        let (target_u, target_v) = generate_target(target);
        let config = GridSearchConfig {
            feed_min: 0.058,
            feed_max: 0.062,
            feed_count: 3,
            kill_min: 0.060,
            kill_max: 0.064,
            kill_count: 3,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
        };

        let result = grid_search(target, config, &target_u, &target_v);

        assert_eq!(result.evaluated, 9);
        assert!((result.best_feed - params.feed).abs() <= f32::EPSILON);
        assert!((result.best_kill - params.kill).abs() <= f32::EPSILON);
        assert_eq!(result.best_loss, 0.0);
    }

    #[test]
    fn grid_search_recovers_close_candidate_for_off_grid_target() {
        let params = GrayScottParams::new(0.06055, 0.06245, 0.16, 0.08, 1.0);
        let target = InverseTarget::new(32, 32, 100, 5, params);
        let (target_u, target_v) = generate_target(target);
        let config = GridSearchConfig {
            feed_min: 0.058,
            feed_max: 0.063,
            feed_count: 11,
            kill_min: 0.060,
            kill_max: 0.065,
            kill_count: 11,
            diff_u: 0.16,
            diff_v: 0.08,
            dt: 1.0,
        };

        let result = grid_search(target, config, &target_u, &target_v);

        assert_eq!(result.evaluated, 121);
        assert!((result.best_feed - params.feed).abs() <= 0.000051);
        assert!((result.best_kill - params.kill).abs() <= 0.000051);
        assert!(result.best_loss > 0.0);
    }
}
