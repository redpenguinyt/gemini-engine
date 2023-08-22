use crate::elements::view::ColChar;
pub mod vec3d;
pub use vec3d::{SpatialAxis, Vec3D};

#[derive(Debug)]
pub struct Face {
    pub v_indexes: Vec<usize>,
    pub fill_char: ColChar,
}

impl Face {
    pub fn new(v_indexes: Vec<usize>, fill_char: ColChar) -> Self {
        Self {
            v_indexes,
            fill_char,
        }
    }
}

pub struct Object3D {
    pub pos: Vec3D,
    pub rotation: Vec3D,
    pub vertices: Vec<Vec3D>,
    pub faces: Vec<Face>,
}

impl Object3D {
    pub fn new(pos: Vec3D, rotation: Vec3D, vertices: Vec<Vec3D>, faces: Vec<Face>) -> Self {
        Self {
            pos: pos,
            rotation: rotation,
            vertices: vertices,
            faces: faces,
        }
    }
}
