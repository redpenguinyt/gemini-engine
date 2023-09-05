//! ## Introduction
//! Gemini is a monospaced ASCII rendering engine, capable of 2D and 3D rendering. This is a loose port of [GeminiEngine](https://github.com/redpenguinyt/GeminiEngine) which was made in Python but was scrapped due to performance limitations.
//!
//! **IMPORTANT**: You HAVE to use a monospace font in the terminal for the engine to render the view properly.
//!
//! Go to [`elements`] for a quick start guide.
//!
//! ## Crate Structure
//! This library is made up of three main crates:
//! - [`gameloop`], which handles the gameloop. See the [`gameloop`] documentation to see how to structure the usual Gemini project.
//! - [`elements`], which handles the printing of various objects to a [`View`](elements::View), the central object in a Gemini project.
//! - [`elements3d`], which handles everything 3D-related. Objects that [`elements3d`] converts to a 2d object will then be printed to the screen by a [`View`](elements::View)

pub mod elements;
pub mod elements3d;
pub mod gameloop;

#[cfg(test)]
mod vec2d_tests {
    use super::*;
    use elements::Vec2D;

    #[test]
    fn add_vec2d() {
        assert_eq!(Vec2D::new(15, -3), Vec2D::new(13, 4) + Vec2D::new(2, -7));
    }

    #[test]
    fn subtract_vec2d() {
        assert_eq!(Vec2D::new(2, -10), Vec2D::new(17, 4) - Vec2D::new(15, 14));
    }

    #[test]
    fn rem_vec2d_over() {
        assert_eq!(Vec2D::new(4, 1), Vec2D::new(9, 11) % Vec2D::new(5, 10))
    }

    #[test]
    fn rem_vec2d_under() {
        assert_eq!(Vec2D::new(4, 1), Vec2D::new(-1, -109) % Vec2D::new(5, 10))
    }

    #[test]
    fn eq_vec2d_both() {
        assert_eq!(Vec2D::new(5, 4), Vec2D::new(5, 4))
    }

    #[test]
    fn eq_vec2d_only_one() {
        assert_ne!(Vec2D::new(5, 2), Vec2D::new(5, 4))
    }

    #[test]
    fn eq_vec2d_neither() {
        assert_ne!(Vec2D::new(17, 2), Vec2D::new(5, 4))
    }
}
