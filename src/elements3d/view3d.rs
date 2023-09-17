use crate::elements::{
    view::{utils, ColChar, Modifier},
    Line, PixelContainer, Point, Polygon, Text, Vec2D,
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

/// The `Viewport` handles printing 3D objects to a 2D [`View`](crate::elements::View), and also acts as the scene's camera.
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
        let sx = (sx * self.character_width_multiplier).round();
        let sy = sy.round();

        self.origin + Vec2D::new(sx as isize, sy as isize)
    }

    /// Return the object's vertices, transformed
    pub fn transform_vertices(&self, object: &impl ViewElement3D) -> Vec<Vec3D> {
        object
            .get_vertices()
            .iter()
            .map(|v| (self.transform * object.get_transform()) * *v)
            .collect()
    }

    /// Return the screen coordinates and distance from the view for each vertex, as parallel vectors
    pub fn get_vertices_on_screen(&self, object: &impl ViewElement3D) -> (Vec<Vec2D>, Vec<f64>) {
        self.transform_vertices(object)
            .iter()
            .map(|vertex| (self.perspective(*vertex), vertex.z))
            .unzip()
    }

    pub fn project_faces(
        &self,
        objects: Vec<&impl ViewElement3D>,
        sort_faces: bool,
        backface_culling: bool,
    ) -> Vec<(Vec<Vec2D>, ColChar)> {
        let mut screen_faces = vec![];

        for object in objects {
            let (screen_coordinates, vertex_depths) = self.get_vertices_on_screen(object);

            for face in object.get_faces().iter() {
                let face_vertices = face.index_into(&screen_coordinates);

                // Backface culling
                if !utils::is_clockwise(&face_vertices) && backface_culling {
                    continue;
                }

                let mean_z = match sort_faces {
                    true => Some(
                        face.index_into(&vertex_depths).into_iter().sum::<f64>()
                            / face_vertices.len() as f64,
                    ),
                    false => None,
                };

                screen_faces.push((face_vertices, face.fill_char, mean_z));
            }
        }

        if sort_faces {
            screen_faces.sort_by_key(|k| (k.2.unwrap() * -100.0).round() as i64);
        }

        screen_faces.into_iter().map(|(vs, c, _)| (vs, c)).collect()
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
                    for (i, screen_coordinates) in
                        self.get_vertices_on_screen(object).0.iter().enumerate()
                    {
                        let index_text = i.to_string();
                        canvas.blit(&Text::new(*screen_coordinates, &index_text, Modifier::None));
                    }
                }
            }
            DisplayMode::Points { fill_char } => {
                for object in objects {
                    for screen_coordinates in self.get_vertices_on_screen(object).0 {
                        canvas.push(Point::new(screen_coordinates, fill_char));
                    }
                }
            }
            DisplayMode::Wireframe { backface_culling } => {
                let screen_faces = self.project_faces(objects, false, backface_culling);

                for (face_vertices, fill_char) in screen_faces {
                    for fi in 0..face_vertices.len() {
                        let (i0, i1) = (
                            face_vertices[fi],
                            face_vertices[(fi + 1) % face_vertices.len()],
                        );
                        canvas.append_points(Line::draw(i0, i1), fill_char);
                    }
                }
            }
            DisplayMode::Solid => {
                let screen_faces = self.project_faces(objects, true, true);

                for (face_vertices, fill_char) in screen_faces {
                    canvas.append_points(Polygon::draw(&face_vertices), fill_char)
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
    fn get_vertices(&self) -> &[Vec3D];
    /// This should return all of the object's `Face`s
    fn get_faces(&self) -> &[Face];
}
