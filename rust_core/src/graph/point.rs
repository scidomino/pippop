use macroquad::math::Vec2;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
        }
    }
}
