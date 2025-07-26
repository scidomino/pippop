// rust_core/src/physics/mod.rs

pub mod force;

// We re-export these functions to make them accessible from the parent `physics` module
// e.g., `crate::physics::calculate_bubble_area` instead of `crate::physics::area::calculate_bubble_area`
mod area;
pub use area::{calculate_bubble_area, calculate_half_area};
