use super::{Face, Transform3D, Vec3D, ViewElement3D};
mod mesh3d_presets;

/// The struct for a Mesh3D object, containing a position, rotation, collection of vertices and collection of [`Face`]s with indices to the vertex collection.
#[derive(Debug, Clone)]
pub struct Mesh3D {
    pub transform: Transform3D,
    /// A vector of the [`Mesh3D`]'s
    pub vertices: Vec<Vec3D>,
    /// A vector of [`Face`]s of indexes into [`Mesh3D::vertices`]
    pub faces: Vec<Face>,
}

impl Mesh3D {
    /// Create a `Mesh3D` with a default `Transform3D`
    pub const fn new(transform: Transform3D, vertices: Vec<Vec3D>, faces: Vec<Face>) -> Self {
        Self {
            transform,
            vertices,
            faces,
        }
    }

    /// Create a `Mesh3D` with a default `Transform3D`
    pub const fn new_at_origin(vertices: Vec<Vec3D>, faces: Vec<Face>) -> Mesh3D {
        Mesh3D {
            transform: Transform3D::DEFAULT,
            vertices,
            faces,
        }
    }
}

impl ViewElement3D for Mesh3D {
    fn get_transform(&self) -> Transform3D {
        self.transform
    }
    fn get_vertices(&self) -> &[Vec3D] {
        &self.vertices
    }
    fn get_faces(&self) -> &[Face] {
        &self.faces
    }
}
