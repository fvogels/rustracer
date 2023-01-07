#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position<const N: usize> {
    pub coords: [i32; N],
}

impl<const N: usize> Position<N> {
    pub fn new(coords: [i32; N]) -> Self {
        Position { coords }
    }
}

impl Position<2> {
    pub fn cartesian(x: i32, y: i32) -> Self {
        Position::new([x, y])
    }

    pub fn x(&self) -> i32 {
        self.coords[0]
    }

    pub fn y(&self) -> i32 {
        self.coords[1]
    }
}

impl Position<3> {
    pub fn cartesian(x: i32, y: i32, z: i32) -> Self {
        Position::new([x, y, z])
    }

    pub fn x(&self) -> i32 {
        self.coords[0]
    }

    pub fn y(&self) -> i32 {
        self.coords[1]
    }

    pub fn z(&self) -> i32 {
        self.coords[2]
    }
}
