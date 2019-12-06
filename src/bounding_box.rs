use crate::coordinate::Coordinate;
use std::fmt::{self, Display};

pub struct BoundingBox {
    pub min: Coordinate,
    pub max: Coordinate,
}

impl Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.rectangle()
                .iter()
                .map(|coordinate| format!(
                    "{{x: {}, y: {}, z: {}}}",
                    coordinate.x, coordinate.y, coordinate.z
                ))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl BoundingBox {
    pub fn zeroed() -> BoundingBox {
        BoundingBox {
            min: Coordinate::origin(),
            max: Coordinate::origin(),
        }
    }

    pub fn rectangle(&self) -> [Coordinate; 8] {
        [
            Coordinate {
                x: self.min.x,
                y: self.min.y,
                z: self.min.z,
            },
            Coordinate {
                x: self.min.x,
                y: self.max.y,
                z: self.min.z,
            },
            Coordinate {
                x: self.min.x,
                y: self.min.y,
                z: self.max.z,
            },
            Coordinate {
                x: self.min.x,
                y: self.max.y,
                z: self.max.z,
            },
            Coordinate {
                x: self.max.x,
                y: self.min.y,
                z: self.min.z,
            },
            Coordinate {
                x: self.max.x,
                y: self.max.y,
                z: self.min.z,
            },
            Coordinate {
                x: self.max.x,
                y: self.min.y,
                z: self.max.z,
            },
            Coordinate {
                x: self.max.x,
                y: self.max.y,
                z: self.max.z,
            },
        ]
    }
}
