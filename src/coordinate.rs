use std::convert::From;

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<(f32, f32, f32)> for Coordinate {
    fn from((x, y, z): (f32, f32, f32)) -> Coordinate {
        Coordinate { x, y, z }
    }
}

impl Coordinate {
    pub fn origin() -> Coordinate {
        Coordinate {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn pairwise_add(&self, other: &Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
