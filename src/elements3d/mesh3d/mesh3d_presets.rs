//! This file contains the presets available when spawning a [`Mesh3D`]

use std::f64::consts::PI;

use crate::{
    elements::view::{ColChar, Modifier},
    elements3d::{Face, Mesh3D, Transform3D, Vec3D},
};

impl Mesh3D {
    /// The gemini_engine equivalent of Blender's default cube. Has side lengths of 2
    pub fn default_cube() -> Self {
        Self::new_at_origin(
            vec![
                Vec3D::new(1.0, 1.0, -1.0),
                Vec3D::new(1.0, 1.0, 1.0),
                Vec3D::new(1.0, -1.0, -1.0),
                Vec3D::new(1.0, -1.0, 1.0),
                Vec3D::new(-1.0, 1.0, -1.0),
                Vec3D::new(-1.0, 1.0, 1.0),
                Vec3D::new(-1.0, -1.0, -1.0),
                Vec3D::new(-1.0, -1.0, 1.0),
            ],
            vec![
                Face::new(vec![2, 3, 1, 0], ColChar::SOLID.with_mod(Modifier::BLUE)),
                Face::new(vec![4, 5, 7, 6], ColChar::SOLID.with_mod(Modifier::BLUE)),
                Face::new(vec![1, 3, 7, 5], ColChar::SOLID.with_mod(Modifier::None)),
                Face::new(vec![4, 6, 2, 0], ColChar::SOLID.with_mod(Modifier::None)),
                Face::new(vec![6, 7, 3, 2], ColChar::SOLID.with_mod(Modifier::RED)),
                Face::new(vec![0, 1, 5, 4], ColChar::SOLID.with_mod(Modifier::RED)),
            ],
        )
    }

    /// A gimbal to help you orient in gemini_engine's 3D space. The orientation is as follows (from the default [`Viewport`](super::Viewport))
    /// - X (red) increases as you move to the right
    /// - Y (green) increases as you move up
    /// - Z (blue) increases as you move away from the viewport
    ///
    /// Think of it like Blender's axes but with Y and Z swapped.
    /// This Mesh does not render in `DisplayMode::SOLID` (see [`DisplayMode`](super::DisplayMode) documentation)
    pub fn gimbal() -> Self {
        Self::new_at_origin(
            vec![
                Vec3D::ZERO,
                Vec3D::new(1.0, 0.0, 0.0),
                Vec3D::new(0.0, 1.0, 0.0),
                Vec3D::new(0.0, 0.0, 1.0),
            ],
            vec![
                Face::new(vec![0, 1], ColChar::SOLID.with_mod(Modifier::RED)),
                Face::new(vec![0, 2], ColChar::SOLID.with_mod(Modifier::GREEN)),
                Face::new(vec![0, 3], ColChar::SOLID.with_mod(Modifier::BLUE)),
            ],
        )
    }
}
