use crate::elements::view::{ColChar, Modifier};

use super::{Face, Transform3D, Vec3D, ViewElement3D};

/// The struct for a Mesh3D object, containing a position, rotation, collection of vertices and collection of [`Face`]s with indices to the vertex collection.
#[derive(Debug, Clone)]
pub struct Mesh3D {
    pub transform: Transform3D,
    pub vertices: Vec<Vec3D>,
    pub faces: Vec<Face>,
}

impl Mesh3D {
    /// The gemini_engine equivalent of Blender's default cube. Has side lengths of 2
    pub fn default_cube() -> Self {
        Self::new(
            Transform3D::DEFAULT,
            vec![
                Vec3D::new(1.0, 1.0, -1.0),
                Vec3D::new(1.0, 1.0, 1.0),
                Vec3D::new(1.0, -1.0, -1.0),
                Vec3D::new(1.0, -1.0, 1.0),
                Vec3D::new(-1.0, 1.0, -1.0),
                Vec3D::new(-1.0, 1.0, 1.0),
                Vec3D::new(-1.0, -1.0, -1.0),
                Vec3D::new(-1.0, -1.0, 1.0),
            ],
            vec![
                Face::new(vec![2, 3, 1, 0], ColChar::SOLID.with_mod(Modifier::BLUE)),
                Face::new(vec![4, 5, 7, 6], ColChar::SOLID.with_mod(Modifier::BLUE)),
                Face::new(vec![1, 3, 7, 5], ColChar::SOLID.with_mod(Modifier::None)),
                Face::new(vec![4, 6, 2, 0], ColChar::SOLID.with_mod(Modifier::None)),
                Face::new(vec![6, 7, 3, 2], ColChar::SOLID.with_mod(Modifier::RED)),
                Face::new(vec![0, 1, 5, 4], ColChar::SOLID.with_mod(Modifier::RED)),
            ],
        )
    }

    /// A gimbal to help you orient in gemini_engine's 3D space. The orientation is as follows (from the default [`Viewport`](super::Viewport))
    /// - X (red) increases as you move to the right
    /// - Y (green) increases as you move up
    /// - Z (blue) increases as you move away from the viewport
    ///
    /// Think of it like Blender's axes but with Y and Z swapped.
    /// This Mesh does not render in `DisplayMode::SOLID` (see [`DisplayMode`](super::DisplayMode) documentation)
    pub fn gimbal() -> Self {
        Self::new(
            Transform3D::DEFAULT,
            vec![
                Vec3D::ZERO,
                Vec3D::new(1.0, 0.0, 0.0),
                Vec3D::new(0.0, 1.0, 0.0),
                Vec3D::new(0.0, 0.0, 1.0),
            ],
            vec![
                Face::new(vec![0, 1], ColChar::SOLID.with_mod(Modifier::RED)),
                Face::new(vec![0, 2], ColChar::SOLID.with_mod(Modifier::GREEN)),
                Face::new(vec![0, 3], ColChar::SOLID.with_mod(Modifier::BLUE)),
            ],
        )
    }

    pub fn new(transform: Transform3D, vertices: Vec<Vec3D>, faces: Vec<Face>) -> Self {
        Self {
            transform,
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
