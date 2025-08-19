
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

impl Coordinate {
    pub fn new(x: f32, y: f32) -> Self {
        Coordinate { x, y }
    }

    pub fn add(&mut self, other: Coordinate) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub position: Coordinate,
    pub velocity: Coordinate,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            position: Coordinate::new(x, y),
            velocity: Coordinate::default(),
        }
    }
}
