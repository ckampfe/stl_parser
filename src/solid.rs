use crate::bounding_box::BoundingBox;
use crate::facet::Facet;

pub struct Solid {
    pub name: Option<Vec<u8>>,
    pub facets: Vec<Facet>,
}

impl Solid {
    /// Does some basic analysis of the solid.
    /// The first return value is the number of triangles in the solid, which can give an idea of the model's roughness
    /// or smoothness.
    /// The second return value is the total surface area of the model (assuming non-overlapping facets).
    /// The last return value is a bounding box approximating a rectangle that just fits the shape.
    pub fn analyze(&self) -> (usize, f32, BoundingBox) {
        let mut total_surface_area = 0f32;
        let mut solid_bounding_box = BoundingBox::zeroed();

        for facet in self.facets.iter() {
            total_surface_area = total_surface_area + facet.surface_area();
            solid_bounding_box = facet.update_bounding_box(solid_bounding_box);
        }

        (self.facets.len(), total_surface_area, solid_bounding_box)
    }
}
