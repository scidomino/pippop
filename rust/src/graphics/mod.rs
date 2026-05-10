//! Rendering and Visual Effects
//!
//! This module provides the tools necessary to translate the logical `Graph` into
//! visual elements. It includes custom triangulation for the fluid bubble shapes
//! and mesh generation for glowing boundaries and ribbons.

pub mod bubble;
pub mod colors;
pub mod geometry;

use crate::{game::state::GamePhase, graph::Graph};
use macroquad::prelude::*;

pub struct RenderContext<'a> {
    pub graph: &'a Graph,
    pub phase: &'a GamePhase,
    pub font: &'a Font,
    pub camera: &'a Camera2D,
}
