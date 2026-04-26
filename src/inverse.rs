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
