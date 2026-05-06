//! PipPop Core Architecture
//!
//! This crate contains the entire logic, physics, and rendering for PipPop.
//! It is designed to be fully platform-agnostic, relying only on Macroquad
//! for graphics, audio, and input.
//!
//! The architecture is split into four primary modules:
//! - [`graph`]: A Half-Edge data structure that maintains the planar topology
//!   of the bubbles.
//! - [`physics`]: A custom Bezier-based physics engine that simulates surface
//!   tension and pressure.
//! - [`game`]: The high-level state machine and specific game logic managers
//!   (e.g., swapping, popping, scoring).
//! - [`graphics`]: Tools for translating the mathematical graph into Macroquad
//!   meshes and drawing them.

pub mod game;
pub mod graph;
pub mod graphics;
pub mod physics;
pub mod resources;
