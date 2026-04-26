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
}
