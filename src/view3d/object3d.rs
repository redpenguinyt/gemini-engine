use crate::elements::view::{ColChar, Vec2D};
pub mod vec3d;
pub use vec3d::{SpatialAxis, Vec3D};

use super::Viewport;

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

impl Clone for Face {
    fn clone(&self) -> Self {
        Self {
            v_indexes: self.v_indexes.clone(),
            fill_char: self.fill_char,
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

    pub fn vertices_on_screen(&self, viewport: &Viewport) -> Vec<(Vec2D, f64)> {
        let mut screen_vertices = vec![];
        for vertex in &self.vertices {
            let pos = vertex.global_position(&viewport, &self);

            let screen_coordinates = viewport.origin + pos.spatial_to_screen(viewport.fov);
            screen_vertices.push((screen_coordinates, pos.z));
        }

        screen_vertices
    }
}

impl Clone for Object3D {
    fn clone(&self) -> Self {
        Self {
            pos: self.pos,
            rotation: self.rotation,
            vertices: self.vertices.clone(),
            faces: self.faces.clone(),
        }
    }
}
