//! Rendering and Visual Effects
//!
//! This module provides the tools necessary to translate the logical `Graph` into
//! visual elements. It includes custom triangulation for the fluid bubble shapes
//! and mesh generation for glowing boundaries and ribbons.

pub mod bubble;
pub mod colors;
pub mod geometry;

use crate::game::state::GameState;
use crate::resources::Resources;
use macroquad::prelude::*;

pub struct RenderContext<'a> {
    pub state: &'a GameState,
    pub resources: &'a Resources,
    pub camera: &'a Camera2D,
}
