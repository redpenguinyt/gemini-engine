use crate::{elements::Vec2D, elements3d::Vec3D};

#[derive(Debug, Clone, Copy)]
pub struct ProjectedVertex {
    pub original: Vec3D,
    pub displayed: Vec2D,
}

impl ProjectedVertex {
    pub const fn new(original: Vec3D, displayed: Vec2D) -> Self {
        Self {
            original,
            displayed,
        }
    }

    pub fn z_index(&self) -> f64 {
        self.original.magnitude()
    }
}
