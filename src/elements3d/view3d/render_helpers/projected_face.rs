use crate::{
    elements::{view::ColChar, Vec2D},
    elements3d::Vec3D,
};

pub struct ProjectedFace {
    /// Where the points appear on the screen
    pub screen_points: Vec<Vec2D>,
    /// The original vertices in 3d space
    pub original_vertices: Vec<Vec3D>,
    /// The vertices' associated z indexes
    pub z_index: Option<f64>,
    /// The face's fill char
    pub fill_char: ColChar,
}

impl ProjectedFace {
    /// Create a new `ProjectedFace`
    pub const fn new(
        screen_points: Vec<Vec2D>,
        original_vertices: Vec<Vec3D>,
        z_index: Option<f64>,
        fill_char: ColChar,
    ) -> Self {
        Self {
            screen_points,
            original_vertices,
            z_index,
            fill_char,
        }
    }

    /// Get the "centre" of the face in 3D space, calculated based on the average of the original vertices
    pub fn get_average_centre(&self) -> Vec3D {
        self.original_vertices.iter().copied().sum::<Vec3D>() / self.original_vertices.len() as f64
    }

    /// Get the normal of the face
    pub fn get_normal(&self) -> Option<Vec3D> {
        if self.original_vertices.len() >= 3 {
            let v0 = self.original_vertices[0] - self.original_vertices[2];
            let v1 = self.original_vertices[1] - self.original_vertices[2];
            Some(v0.cross(v1).normal())
        } else {
            None
        }
    }
}
