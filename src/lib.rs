pub mod solver;

pub use solver::{GrayScott, GrayScottParams};

#[cfg(target_arch = "wasm32")]
mod wasm;
