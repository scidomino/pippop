// rust_core/src/physics/mod.rs

pub mod force;
pub mod area;

// Re-export only the top-level functions that the simulation needs.
pub use area::calculate_bubble_area;