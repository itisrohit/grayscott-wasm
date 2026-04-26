pub mod inverse;
pub mod solver;

pub use inverse::{
    add_uniform_noise, finite_difference_gradient, forward_gradient, generate_target,
    gradient_descent, grid_search, loss_for_params, FiniteDifferenceGradient, ForwardGradient,
    GradientDescentConfig, GradientDescentResult, GradientDescentStep, GridSearchConfig,
    GridSearchResult, InverseTarget,
};
pub use solver::{GrayScott, GrayScottParams};

#[cfg(target_arch = "wasm32")]
mod wasm;
