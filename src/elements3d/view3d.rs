pub mod face;
pub mod vec3d;
use crate::elements::{
    view::{
        utils::{self, points_to_pixels, Wrapping},
        ColChar, Modifier,
    },
    Line, PixelContainer, Polygon, Sprite, Vec2D, View,
};
pub use face::Face;
pub use vec3d::{SpatialAxis, Vec3D};

/// `DisplayMode` determines how the `Viewport` renders our 3D objects. This is the Gemini equivalent of Blender's Viewport Shading options
/// - `DisplayMode::Points` only renders the object's vertices as single pixels with the `ColChar` chosen with the fill_char enum parameter
/// - `DisplayMode::Debug` does the same thing, but shows the vertices as the indices that represent them (this is useful when you are constructing a mesh)
///
pub enum DisplayMode {
    Solid,
    Wireframe,
    Points { fill_char: ColChar },
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

    pub fn blit_to<T: ViewElement3D>(
        &self,
        view: &mut View,
        objects: Vec<&T>,
        display_mode: DisplayMode,
    ) {
        match display_mode {
            DisplayMode::Debug => {
                for object in objects {
                    for (i, (screen_coordinates, _z)) in
                        object.vertices_on_screen(self).iter().enumerate()
                    {
                        let index_text = format!("{}", i);
                        view.blit(
                            &Sprite::new(*screen_coordinates, index_text.as_str(), Modifier::None),
                            Wrapping::Ignore,
                        );
                    }
                }
            }
            DisplayMode::Points { fill_char } => {
                for object in objects {
                    for (screen_coordinates, _z) in object.vertices_on_screen(self) {
                        view.plot(screen_coordinates, fill_char, Wrapping::Ignore);
                    }
                }
            }
            DisplayMode::Wireframe => {
                for object in objects {
                    let screen_vertices = object.vertices_on_screen(&self);

                    for face in (*object.get_faces()).into_iter() {
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
            }
            DisplayMode::Solid => {
                let mut screen_faces = vec![];

                for object in objects {
                    let screen_vertices = object.vertices_on_screen(&self);

                    for face in (*object.get_faces()).into_iter() {
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

                        screen_faces.push((vertices_only, mean_z, face.fill_char));
                    }
                }

                screen_faces.sort_by_key(|k| (k.1 * 100.0).round() as isize);

                for face in screen_faces {
                    let polygon = Polygon::new(face.0, face.2);
                    view.blit(&polygon, Wrapping::Ignore)
                }
            }
        }
    }
}

pub trait ViewElement3D {
    /// This should return the position of the object's origin point
    fn get_pos(&self) -> Vec3D;
    /// This should return the rotation of the object's origin point
    fn get_rotation(&self) -> Vec3D;
    /// This should return all of the object's vertices
    fn get_vertices(&self) -> Vec<Vec3D>;
    /// This should return all of the object's `Face`s
    fn get_faces(&self) -> Vec<Face>;
    /// This should return a list of its vertices in their screen positions, paired with their distance to the screen
    fn vertices_on_screen(&self, viewport: &Viewport) -> Vec<(Vec2D, f64)>;
}
