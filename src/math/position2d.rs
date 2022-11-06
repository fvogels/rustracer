#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position2D {
    pub x: i32,
    pub y: i32,
}

impl Position2D {
    pub fn new(x: i32, y: i32) -> Self {
        Position2D { x, y }
    }
}
