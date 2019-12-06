use crate::bounding_box::BoundingBox;
use crate::coordinate::Coordinate;

#[derive(Clone, Debug)]
pub struct Facet {
    pub normal_vector: (f32, f32, f32),
    pub vertices: (Coordinate, Coordinate, Coordinate),
}

impl Facet {
    pub fn update_bounding_box(&self, bounding_box: BoundingBox) -> BoundingBox {
        let (a, b, c) = self.vertices;
        BoundingBox {
            min: Coordinate {
                x: bounding_box.min.x.min(a.x.min(b.x.min(c.x))),
                y: bounding_box.min.y.min(a.y.min(b.y.min(c.y))),
                z: bounding_box.min.z.min(a.z.min(b.z.min(c.z))),
            },
            max: Coordinate {
                x: bounding_box.max.x.max(a.x.max(b.x.max(c.x))),
                y: bounding_box.max.y.max(a.y.max(b.y.max(c.y))),
                z: bounding_box.max.z.max(a.z.max(b.z.max(c.z))),
            },
        }
    }

    /// Finds the surface area of this facet.
    /// Note that a solid may have overlapping facets, so some of this surface area may not count towards the total for
    /// the solid.
    /// Solution taken from https://math.stackexchange.com/a/128999
    pub fn surface_area(&self) -> f32 {
        let (a, b, c) = self.vertices;
        let ab = a.pairwise_add(&b);
        let ac = a.pairwise_add(&c);
        (((ab.x * ac.y) - (ab.y * ac.x)).powi(2)
            + ((ab.y * ac.z) - (ab.z * ac.y)).powi(2)
            + ((ab.z * ac.x) - (ab.x * ac.z)).powi(2))
        .sqrt()
            * 0.5
    }
}
