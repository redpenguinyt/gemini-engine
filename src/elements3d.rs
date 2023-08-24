//! Gemini's implementation of 3D rendering. Experimental
use crate::elements::view::vec2d::Vec2D;
pub mod view3d;
pub use view3d::{DisplayMode, Face, SpatialAxis, Vec3D, ViewElement3D, Viewport};

pub struct Mesh3D {
    pub pos: Vec3D,
    pub rotation: Vec3D,
    pub vertices: Vec<Vec3D>,
    pub faces: Vec<Face>,
}

impl Mesh3D {
    pub fn new(pos: Vec3D, rotation: Vec3D, vertices: Vec<Vec3D>, faces: Vec<Face>) -> Self {
        Self {
            pos: pos,
            rotation: rotation,
            vertices: vertices,
            faces: faces,
        }
    }
}

impl Clone for Mesh3D {
    fn clone(&self) -> Self {
        Self {
            pos: self.pos,
            rotation: self.rotation,
            vertices: self.vertices.clone(),
            faces: self.faces.clone(),
        }
    }
}

impl ViewElement3D for Mesh3D {
    fn get_pos(&self) -> Vec3D {
        self.pos.clone()
    }
    fn get_rotation(&self) -> Vec3D {
        self.rotation.clone()
    }
    fn get_vertices(&self) -> Vec<Vec3D> {
        self.vertices.clone()
    }
    fn get_faces(&self) -> Vec<Face> {
        self.faces.clone()
    }
    fn vertices_on_screen(&self, viewport: &Viewport) -> Vec<(Vec2D, f64)> {
        let mut screen_vertices = vec![];
        for vertex in &self.vertices {
            let pos = vertex.global_position(&viewport, self);

            let screen_coordinates = viewport.origin + pos.spatial_to_screen(viewport.fov);
            screen_vertices.push((screen_coordinates, pos.z));
        }

        screen_vertices
    }
}
