use crate::elements::{view::ColChar, Vec2D};

use super::Vec3D;

/// A Face contains indices to a mesh's collection of vertices and a fill_char to fill the face. Indices should be arranged in a clockwise order, as if they appear counter-clockwise when rendering they will not be rendered at all (this is how gemini-engine handles backface culling and maximises performance)
#[derive(Debug, Clone)]
pub struct IndexFace {
    /// The vertex indices of the face
    pub v_indices: Vec<usize>,
    /// The desired appearance of the face when rendered
    pub fill_char: ColChar,
}

impl IndexFace {
    /// Create a new face with the given indexes and [`ColChar`]
    pub const fn new(v_indices: Vec<usize>, fill_char: ColChar) -> Self {
        Self {
            v_indices,
            fill_char,
        }
    }

    /// Returns a vector with the elements found at the vertex indices of the given slice
    pub fn index_into<T: Copy>(&self, vertices: &[T]) -> Vec<T> {
        // TODO: make sure the list of vertices is long enough to index into
        // TODO: return a slice here instead
        self.v_indices.iter().map(|vi| vertices[*vi]).collect()
    }
}

pub struct ProjectedFace {
    pub screen_points: Vec<Vec2D>,
    pub original_vertices: Vec<Vec3D>,
    pub z_index: f64,
    pub fill_char: ColChar,
}

impl ProjectedFace {
    pub fn new(
        screen_points: Vec<Vec2D>,
        original_vertices: Vec<Vec3D>,
        z_index: f64,
        fill_char: ColChar,
    ) -> Self {
        Self {
            screen_points,
            original_vertices,
            z_index,
            fill_char,
        }
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
