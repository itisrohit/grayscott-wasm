pub mod inverse;
pub mod solver;

pub use inverse::{
    finite_difference_gradient, generate_target, grid_search, loss_for_params,
    FiniteDifferenceGradient, GridSearchConfig, GridSearchResult, InverseTarget,
};
pub use solver::{GrayScott, GrayScottParams};

#[cfg(target_arch = "wasm32")]
mod wasm;
