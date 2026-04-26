use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use grayscott_wasm::{
    finite_difference_gradient, forward_gradient, generate_target, loss_for_params,
    GrayScottParams, InverseTarget,
};
use std::hint::black_box;

#[derive(Debug, Clone)]
struct Case {
    target: InverseTarget,
    guess_params: GrayScottParams,
    target_u: Vec<f32>,
    target_v: Vec<f32>,
    epsilon: f32,
}

impl Case {
    fn new(grid: usize) -> Self {
        let target_params = GrayScottParams::new(0.06055, 0.06245, 0.16, 0.08, 1.0);
        let target = InverseTarget::new(grid, grid, 100, 5, target_params);
        let guess_params = GrayScottParams::new(0.060, 0.063, 0.16, 0.08, 1.0);
        let (target_u, target_v) = generate_target(target);

        Self {
            target,
            guess_params,
            target_u,
            target_v,
            epsilon: 1.0e-4,
        }
    }
}

fn inverse_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("inverse_overhead");
    group
        .sample_size(10)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2));

    for grid in [128_usize, 256] {
        let case = Case::new(grid);

        group.bench_with_input(BenchmarkId::new("primal_loss", grid), &case, |b, case| {
            b.iter(|| {
                black_box(loss_for_params(
                    case.target,
                    case.guess_params,
                    &case.target_u,
                    &case.target_v,
                ))
            });
        });

        group.bench_with_input(
            BenchmarkId::new("finite_difference_gradient", grid),
            &case,
            |b, case| {
                b.iter(|| {
                    let gradient = finite_difference_gradient(
                        case.target,
                        case.guess_params,
                        &case.target_u,
                        &case.target_v,
                        case.epsilon,
                    );
                    black_box(gradient.base_loss + gradient.feed + gradient.kill)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("forward_mode_ad_gradient", grid),
            &case,
            |b, case| {
                b.iter(|| {
                    let gradient = forward_gradient(
                        case.target,
                        case.guess_params,
                        &case.target_u,
                        &case.target_v,
                    );
                    black_box(gradient.loss + gradient.feed + gradient.kill)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, inverse_overhead);
criterion_main!(benches);
