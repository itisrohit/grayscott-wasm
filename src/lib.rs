pub mod inverse;
pub mod solver;

pub use inverse::{
    generate_target, grid_search, GridSearchConfig, GridSearchResult, InverseTarget,
};
pub use solver::{GrayScott, GrayScottParams};

#[cfg(target_arch = "wasm32")]
mod wasm;
