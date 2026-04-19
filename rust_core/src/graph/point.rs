use macroquad::math::Vec2;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self::from_vec2(macroquad::math::vec2(x, y))
    }
    pub fn from_vec2(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
        }
    }
}
