use macroquad::math::{vec2, Vec2};

/// A physical point in 2D space, used by the custom physics engine.
///
/// This represents either a `Vertex` (junction) or the control point of an `Edge`.
/// The physics solver accumulates forces into the `velocity` and integrates it
/// into the `position` every frame.
#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self::from_vec2(vec2(x, y))
    }
    pub fn from_vec2(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
        }
    }
}
