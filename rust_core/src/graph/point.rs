
#[derive(Debug, Default, Clone, Copy)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub position: Coordinate,
    pub velocity: Coordinate,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            position: Coordinate{x, y},
            velocity: Coordinate{x: 0.0, y: 0.0},
        }
    }
}
