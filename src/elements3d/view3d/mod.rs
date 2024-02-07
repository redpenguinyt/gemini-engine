//! This module is home to the [`Viewport`], which handles the projecting of [`ViewElement3D`]s to a format then displayable by a [`View`](crate::elements::View)

use crate::elements::{
    view::{utils, ColChar, Modifier},
    Line, Pixel, PixelContainer, Polygon, Text, Vec2D,
};
mod display_mode;
mod render_helpers;
mod transform3d;
pub use display_mode::{
    lighting::{Light, LightType, BRIGHTNESS_CHARS},
    DisplayMode,
};
pub use render_helpers::Face;
use render_helpers::ProjectedFace;
pub use transform3d::{Transform3D, Vec3D};

use self::render_helpers::ProjectedVertex;

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
    /// Any face with vertices closer to the viewport than this value will be clipped
    pub clipping_distace: f64,
}

impl Viewport {
    /// Create a new Viewport with a default [`character_width_multiplier`](Viewport::character_width_multiplier) of 2.2
    #[must_use]
    pub const fn new(transform: Transform3D, fov: f64, screen_origin: Vec2D) -> Self {
        Self {
            transform,
            fov,
            origin: screen_origin,
            character_width_multiplier: 2.2,
            clipping_distace: 0.3,
        }
    }

    /// Project the [`Vec3D`] on a flat plane using the `Viewport`'s [fov](Viewport::fov) and [`character_width_multiplier`](Viewport::character_width_multiplier)
    fn perspective(&self, pos: Vec3D) -> Vec2D {
        let f = self.fov / pos.z;
        let (sx, sy) = (pos.x * f, pos.y * f);

        // adjust for non-square pixels
        let sx = (sx * self.character_width_multiplier).round();
        let sy = sy.round();

        self.origin + Vec2D::new(sx as isize, sy as isize)
    }

    /// Return the object's vertices, transformed
    fn transform_vertices(&self, object: &dyn ViewElement3D) -> Vec<Vec3D> {
        let obj_transformed = object.get_transform().apply_to(object.get_vertices());

        self.transform.apply_viewport_transform(&obj_transformed)
    }

    /// Return the screen coordinates and distance from the view for each vertex, as parallel vectors
    fn get_vertices_on_screen(&self, object: &dyn ViewElement3D) -> Vec<ProjectedVertex> {
        self.transform_vertices(object)
            .into_iter()
            .map(|vertex| ProjectedVertex::new(vertex, self.perspective(vertex)))
            .collect()
    }

    /// Project the faces onto a 2D plane. Returns a collection of faces, each stored as a list of the points it appears at, the normal of the face and the [`ColChar`] assigned to it
    fn project_faces(
        &self,
        objects: Vec<&dyn ViewElement3D>,
        sort_faces: bool,
        backface_culling: bool,
    ) -> Vec<ProjectedFace> {
        let mut screen_faces = vec![];

        for object in objects {
            let vertices = self.get_vertices_on_screen(object);

            for face in object.get_faces() {
                let face_vertices = face.index_into(&vertices);
                let face_screen_points: Vec<Vec2D> =
                    face_vertices.iter().map(|v| v.displayed).collect();

                // Backface culling
                if !utils::is_clockwise(&face_screen_points) && backface_culling {
                    continue;
                }

                // Do not render if behind player
                if face_vertices
                    .iter()
                    .any(|v| v.original.z >= -self.clipping_distace)
                {
                    continue;
                }

                let mean_z = if sort_faces {
                    Some(
                        face_vertices
                            .iter()
                            .map(ProjectedVertex::z_index)
                            .sum::<f64>()
                            / face_vertices.len() as f64,
                    )
                } else {
                    None
                };

                screen_faces.push(ProjectedFace::new(
                    face_screen_points,
                    face_vertices.iter().map(|v| v.original).collect(),
                    mean_z,
                    face.fill_char,
                ));
            }
        }

        if sort_faces {
            screen_faces
                .sort_by_key(|face| (face.z_index.unwrap_or(0.0) * -1000.0).round() as isize);
        }

        screen_faces
    }

    /// Render the objects (implementing [`ViewElement3D`]) given the `Viewport`'s properties. Returns a [`PixelContainer`] which can then be blit to a [`View`](`crate::elements::View`)
    #[must_use]
    pub fn render(
        &self,
        objects: Vec<&dyn ViewElement3D>,
        display_mode: DisplayMode,
    ) -> PixelContainer {
        let mut canvas = PixelContainer::new();

        match display_mode {
            DisplayMode::Debug => {
                for object in objects {
                    for (i, vertex) in self.get_vertices_on_screen(object).iter().enumerate() {
                        let index_text = i.to_string();
                        canvas.blit(&Text::new(vertex.displayed, &index_text, Modifier::None));
                    }
                }
            }
            DisplayMode::Points { fill_char } => {
                for object in objects {
                    for vertex in self.get_vertices_on_screen(object) {
                        canvas.push(Pixel::new(vertex.displayed, fill_char));
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
                        canvas.append_points(&Line::draw(i0, i1), face.fill_char);
                    }
                }
            }
            DisplayMode::Solid => {
                let screen_faces = self.project_faces(objects, true, true);

                for face in screen_faces {
                    canvas.append_points(&Polygon::draw(&face.screen_points), face.fill_char);
                }
            }
            DisplayMode::Illuminated { lights } => {
                let screen_faces = self.project_faces(objects, true, true);

                let brightness_chars: Vec<char> = BRIGHTNESS_CHARS.chars().collect();
                let len_brightness_chars: f64 = brightness_chars.len() as f64;

                for face in screen_faces {
                    let fill_char = if let Some(normal) = face.get_normal() {
                        let intensity: f64 = lights
                            .iter()
                            .map(|light| {
                                light.calculate_intensity(face.get_average_centre(), normal)
                            })
                            .sum();

                        let brightness_char_index = ((intensity * len_brightness_chars).round()
                            as usize)
                            .clamp(0, brightness_chars.len() - 1);
                        let intensity_char = brightness_chars[brightness_char_index];

                        ColChar::new(intensity_char, face.fill_char.modifier)
                    } else {
                        face.fill_char
                    };

                    canvas.append_points(&Polygon::draw(&face.screen_points), fill_char);
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
