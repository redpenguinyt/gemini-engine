//! Gemini's implementation of 3D rendering. Experimental
pub mod object3d;
use crate::elements::{
    view::{
        utils::{self, points_to_pixels, Wrapping},
        ColChar, Modifier,
    },
    Line, PixelContainer, Sprite, Triangle, Vec2D, View,
};
pub use object3d::{Face, Object3D, SpatialAxis, Vec3D};

pub enum DisplayMode {
    Solid,
    Wireframe,
    Points,
    Debug,
}

pub struct Viewport {
    pub offset: Vec3D,
    pub rotation: Vec3D,
    pub fov: f64,
    pub origin: Vec2D,
}

impl Viewport {
    pub fn new(offset: Vec3D, rotation: Vec3D, fov: f64, origin: Vec2D) -> Self {
        Self {
            offset,
            rotation,
            fov,
            origin,
        }
    }

    pub fn blit_to(&self, view: &mut View, object: &Object3D, display_mode: DisplayMode) {
        match display_mode {
            DisplayMode::Debug => {
                for (i, vertex) in (&object.vertices).iter().enumerate() {
                    let pos = vertex.global_position(&self, &object);

                    let screen_coordinates = self.origin + pos.spatial_to_screen(self.fov);
                    let index_text = format!("{}", i);
                    view.blit(
                        &Sprite::new(screen_coordinates, index_text.as_str(), Modifier::None),
                        Wrapping::Ignore,
                    );
                }
            }
            DisplayMode::Points => {
                for vertex in &object.vertices {
                    let pos = vertex.global_position(&self, &object);

                    let screen_coordinates = self.origin + pos.spatial_to_screen(self.fov);
                    view.plot(screen_coordinates, ColChar::SOLID, Wrapping::Ignore);
                }
            }
            DisplayMode::Wireframe => {
                let screen_vertices = object.vertices_on_screen(&self);

                for face in (*object.faces).into_iter() {
                    let mut pixel_container = PixelContainer::new();
                    for fi in 0..face.v_indexes.len() {
                        let (i0, i1) = (
                            face.v_indexes[fi],
                            face.v_indexes[(fi + 1) % face.v_indexes.len()],
                        );
                        pixel_container.append(&mut points_to_pixels(
                            Line::draw(screen_vertices[i0].0, screen_vertices[i1].0),
                            face.fill_char,
                        ));
                    }

                    view.blit(&pixel_container, Wrapping::Ignore)
                }
            }
            DisplayMode::Solid => {
                let screen_vertices = object.vertices_on_screen(&self);

                let mut sorted_faces = vec![];

                for face in (*object.faces).into_iter() {
                    let mut face_vertices = vec![];
                    for vi in &face.v_indexes {
                        face_vertices.push(screen_vertices[*vi]);
                    }
                    let vertices_only = face_vertices.iter().map(|k| k.0).collect();

                    if !utils::is_clockwise(&vertices_only) {
                        continue;
                    }

                    let mut mean_z: f64 = 0.0;
                    for (_v, z) in &face_vertices {
                        mean_z += z;
                    }
                    mean_z /= face_vertices.len() as f64;

                    sorted_faces.push((vertices_only, mean_z, face.fill_char));
                }

                sorted_faces.sort_by_key(|k| (k.1 * 100.0).round() as isize);

                for face in sorted_faces {
                    let mut pixel_container = PixelContainer::new();
                    let face_vertices = face.0;
                    for fi in 1..face_vertices.len() {
                        pixel_container.append(&mut points_to_pixels(
                            Triangle::draw([
                                face_vertices[0],
                                face_vertices[fi],
                                face_vertices[(fi + 1) % face_vertices.len()],
                            ]),
                            face.2,
                        ))
                    }
                    view.blit(&pixel_container, Wrapping::Ignore)
                }
            }
        }
    }
}
