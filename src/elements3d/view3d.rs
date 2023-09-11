use crate::elements::{
    view::{utils, ColChar, Modifier},
    Line, PixelContainer, Point, Polygon, Sprite, Vec2D,
};
pub mod face;
pub mod transform3d;
pub mod vec3d;
pub use face::Face;
pub use transform3d::Transform3D;
pub use vec3d::Vec3D;

/// `DisplayMode` determines how the [`Viewport`] renders our 3D objects. This is the Gemini equivalent of Blender's Viewport Shading options
/// - [`DisplayMode::Debug`] does the same thing, but shows the vertices as the indices that represent them (this is useful when you are constructing a mesh)
/// - [`DisplayMode::Points`] only renders the object's vertices as single pixels with the [`ColChar`] chosen with the `fill_char` enum parameter
/// - [`DisplayMode::Wireframe`] renders the edges of the meshes, without filling in the shapes. You can choose whether you want to render with backface culling using the `backface_culling` enum parameter
/// - [`DisplayMode::Solid`] renders the full faces of all the meshes. This is normally the final render
pub enum DisplayMode {
    Debug,
    Points { fill_char: ColChar },
    Wireframe { backface_culling: bool },
    Solid,
}

/// The `Viewport` handles printing 3D objects to a 2D [`View`], and also acts as the scene's camera.
pub struct Viewport {
    /// How the Viewport is oriented in the 3D scene
    pub transform: Transform3D,
    /// The Viewport's field of view
    pub fov: f64,
    /// The center of the view you intend to print to. You can use `View.center()` as the input for this
    pub origin: Vec2D,
    /// Most terminals don't have perfectly square characters. The value you set here is how much the final image will be stretched in the X axis to account for this. The default value is `2.2` but it will be different in most terminals
    pub character_width_multiplier: f64,
}

impl Viewport {
    pub fn new(transform: Transform3D, fov: f64, origin: Vec2D) -> Self {
        Self {
            transform,
            fov,
            origin,
            character_width_multiplier: 2.2,
        }
    }

    pub fn perspective(&self, pos: Vec3D) -> Vec2D {
        let f = self.fov / -pos.z;
        let (sx, sy) = (-pos.x * f, pos.y * f);

        // adjust for non-square pixels
        let sx = (sx * self.character_width_multiplier).round() as isize;
        let sy = sy.round() as isize;

        self.origin + Vec2D::new(sx, sy)
    }

    /// Return the object's vertices, transformed
    pub fn transform_vertices(&self, object: &impl ViewElement3D) -> Vec<Vec3D> {
        object
            .get_vertices()
            .iter()
            .map(|v| (self.transform * object.get_transform()) * *v)
            .collect()
    }

    /// Return all the screen coordinates for each vertex, paired with the distance from the view
    pub fn get_vertices_on_screen(&self, object: &impl ViewElement3D) -> Vec<(Vec2D, f64)> {
        self.transform_vertices(object)
            .iter()
            .map(|vertex| (self.perspective(*vertex), vertex.z))
            .collect()
    }

    pub fn render(
        &self,
        objects: Vec<&impl ViewElement3D>,
        display_mode: DisplayMode,
    ) -> PixelContainer {
        let mut canvas = PixelContainer::new();

        match display_mode {
            DisplayMode::Debug => {
                for object in objects {
                    for (i, (screen_coordinates, _z)) in
                        self.get_vertices_on_screen(object).iter().enumerate()
                    {
                        let index_text = format!("{}", i);
                        canvas.blit(&Sprite::new(
                            *screen_coordinates,
                            index_text.as_str(),
                            Modifier::None,
                        ));
                    }
                }
            }
            DisplayMode::Points { fill_char } => {
                for object in objects {
                    for (screen_coordinates, _z) in self.get_vertices_on_screen(object) {
                        canvas.push(Point::new(screen_coordinates, fill_char));
                    }
                }
            }
            DisplayMode::Wireframe { backface_culling } => {
                for object in objects {
                    let screen_vertices = self.get_vertices_on_screen(object);

                    for face in (object.get_faces()).into_iter() {
                        if backface_culling {
                            let face_vertex_indices = face
                                .v_indexes
                                .iter()
                                .map(|vi| screen_vertices[*vi].0)
                                .collect();
                            // Backface culling
                            if !utils::is_clockwise(&face_vertex_indices) {
                                continue;
                            }
                        }

                        for fi in 0..face.v_indexes.len() {
                            let (i0, i1) = (
                                face.v_indexes[fi],
                                face.v_indexes[(fi + 1) % face.v_indexes.len()],
                            );
                            canvas.append(&mut utils::points_to_pixels(
                                Line::draw(screen_vertices[i0].0, screen_vertices[i1].0),
                                face.fill_char,
                            ));
                        }
                    }
                }
            }
            DisplayMode::Solid => {
                let mut screen_faces = vec![];

                for object in objects {
                    let screen_vertices = self.get_vertices_on_screen(object);

                    for face in (object.get_faces()).into_iter() {
                        let face_vertex_indices: Vec<(Vec2D, f64)> = face
                            .v_indexes
                            .iter()
                            .map(|vi| screen_vertices[*vi])
                            .collect();
                        let vertices_only = face_vertex_indices.iter().map(|k| k.0).collect();

                        // Backface culling
                        if !utils::is_clockwise(&vertices_only) {
                            continue;
                        }

                        let mut mean_z: f64 = 0.0;
                        for (_v, z) in &face_vertex_indices {
                            mean_z += z;
                        }
                        mean_z /= face_vertex_indices.len() as f64;

                        screen_faces.push((vertices_only, mean_z, face.fill_char));
                    }
                }

                screen_faces.sort_by_key(|k| (k.1 * -100.0).round() as isize);

                for face in screen_faces {
                    let polygon = Polygon::new(face.0, face.2);
                    canvas.blit(&polygon)
                }
            }
        }

        canvas
    }
}

pub trait ViewElement3D {
    /// This should return the object's transform
    fn get_transform(&self) -> Transform3D;
    /// This should return all of the object's vertices
    fn get_vertices(&self) -> &Vec<Vec3D>;
    /// This should return all of the object's `Face`s
    fn get_faces(&self) -> &Vec<Face>;
}
