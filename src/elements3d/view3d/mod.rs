//! This module is home to the [`Viewport`], which handles the projecting of [`ViewElement3D`]s to a format then displayable by a [`View`](crate::elements::View)

use crate::elements::{
    view::{utils, ColChar, Modifier},
    Line, Pixel, PixelContainer, Polygon, Text, Vec2D,
};
mod display_mode;
mod face;
mod light;
mod transform3d;
pub use display_mode::DisplayMode;
pub use face::{IndexFace as Face, ProjectedFace};
pub use light::{Light, LightType, BRIGHTNESS_CHARS};
pub use transform3d::{Transform3D, Vec3D};

/// The `Viewport` handles printing 3D objects to a 2D [`View`](crate::elements::View), and also acts as the scene's camera.
pub struct Viewport {
    /// How the Viewport is oriented in the 3D scene
    pub transform: Transform3D,
    /// The Viewport's field of view
    pub fov: f64,
    /// The center of the view you intend to print to. `View.center()` returns exactly what you need for this
    pub origin: Vec2D,
    /// Most terminals don't have perfectly square characters. The value you set here is how much the final image will be stretched in the X axis to account for this. The default value is `2.2` but it will be different in most terminals
    pub character_width_multiplier: f64,
}

impl Viewport {
    pub const fn new(transform: Transform3D, fov: f64, screen_origin: Vec2D) -> Self {
        Self {
            transform,
            fov,
            origin: screen_origin,
            character_width_multiplier: 2.2,
        }
    }

    /// Project the [`Vec3D`] on a flat plane using the `Viewport`'s [fov](Viewport::fov) and [character_width_multiplier](Viewport::character_width_multiplier)
    pub fn perspective(&self, pos: Vec3D) -> Vec2D {
        let f = self.fov / -pos.z;
        let (sx, sy) = (-pos.x * f, pos.y * f);

        // adjust for non-square pixels
        let sx = (sx * self.character_width_multiplier).round();
        let sy = sy.round();

        self.origin + Vec2D::new(sx as isize, sy as isize)
    }

    /// Return the object's vertices, transformed
    pub fn transform_vertices(&self, object: &dyn ViewElement3D) -> Vec<Vec3D> {
        (self.transform * object.get_transform()).apply_to(object.get_vertices())
    }

    /// Return the screen coordinates and distance from the view for each vertex, as parallel vectors
    pub fn get_vertices_on_screen(&self, object: &dyn ViewElement3D) -> (Vec<Vec2D>, Vec<f64>) {
        self.transform_vertices(object)
            .iter()
            .map(|vertex: &Vec3D| (self.perspective(*vertex), vertex.magnitude()))
            .unzip()
    }

    /// Project the faces onto a 2D plane. Returns a collection of faces, each stored as a list of the points it appears at, the normal of the face and the [`ColChar`] assigned to it
    pub fn project_faces(
        &self,
        objects: Vec<&dyn ViewElement3D>,
        sort_faces: bool,
        backface_culling: bool,
    ) -> Vec<ProjectedFace> {
        let mut screen_faces = vec![];

        for object in objects {
            let (screen_coordinates, vertex_depths) = self.get_vertices_on_screen(object);

            for face in object.get_faces() {
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

                let original_vertices = face.index_into(&self.transform_vertices(object));

                screen_faces.push(ProjectedFace::new(
                    face_vertices,
                    original_vertices,
                    mean_z,
                    face.fill_char,
                ));
            }
        }

        if sort_faces {
            screen_faces.sort_by_key(|face| (face.z_index.unwrap_or(0.0) * -1000.0).round() as i64);
        }

        screen_faces
    }

    /// Render the objects (implementing [`ViewElement3D`]) given the `Viewport`'s properties. Returns a [`PixelContainer`] which can then be blit to a [`View`](`crate::elements::View`)
    pub fn render(
        &self,
        objects: Vec<&dyn ViewElement3D>,
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
                        canvas.push(Pixel::new(screen_coordinates, fill_char));
                    }
                }
            }
            DisplayMode::Wireframe { backface_culling } => {
                let screen_faces = self.project_faces(objects, false, backface_culling);

                for face in screen_faces {
                    for fi in 0..face.screen_points.len() {
                        let (i0, i1) = (
                            face.screen_points[fi],
                            face.screen_points[(fi + 1) % face.screen_points.len()],
                        );
                        canvas.append_points(Line::draw(i0, i1), face.fill_char);
                    }
                }
            }
            DisplayMode::Solid => {
                let screen_faces = self.project_faces(objects, true, true);

                for face in screen_faces {
                    canvas.append_points(Polygon::draw(&face.screen_points), face.fill_char)
                }
            }
            DisplayMode::Illuminated { lights } => {
                let screen_faces = self.project_faces(objects, true, true);

                let brightness_chars: Vec<char> = BRIGHTNESS_CHARS.chars().collect();

                for face in screen_faces {
                    let fill_char = if let Some(normal) = face.get_normal() {
                        let intensity: f64 = lights
                            .iter()
                            .map(|light| light.calculate_intensity(normal))
                            .sum();

                        let intensity_char =
                            brightness_chars[((intensity * brightness_chars.len() as f64).round()
                                as usize)
                                .clamp(0, brightness_chars.len() - 1)];

                        ColChar::new(intensity_char, face.fill_char.modifier)
                    } else {
                        face.fill_char
                    };

                    canvas.append_points(Polygon::draw(&face.screen_points), fill_char);
                }
            }
        }

        canvas
    }
}

/// `ViewElement3D` is a trait that must be implemented by any 3D object to be rendered using a [`Viewport`]
pub trait ViewElement3D {
    /// This should return the object's transform
    fn get_transform(&self) -> Transform3D;
    /// This should return all of the object's vertices
    fn get_vertices(&self) -> &[Vec3D];
    /// This should return all of the object's `Face`s
    fn get_faces(&self) -> &[Face];
}
