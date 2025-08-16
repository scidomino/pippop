#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub position: (f32, f32),
    pub velocity: (f32, f32),
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            position: (x, y),
            velocity: (0.0, 0.0),
        }
    }
}
