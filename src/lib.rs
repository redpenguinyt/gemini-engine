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
mod vec2_tests {
    use super::*;
    use elements::view::vec2d::Vector2;

    #[test]
    fn add_vec2() {
        assert_eq!(
            Vector2::new(15, -3),
            Vector2::new(13, 4) + Vector2::new(2, -7)
        );
    }

    #[test]
    fn subtract_vec2() {
        assert_eq!(
            Vector2::new(2, -10),
            Vector2::new(17, 4) - Vector2::new(15, 14)
        );
    }

    #[test]
    fn rem_vec2_over() {
        assert_eq!(
            Vector2::new(4, 1),
            Vector2::new(9, 11) % Vector2::new(5, 10)
        )
    }

    #[test]
    fn rem_vec2_under() {
        assert_eq!(
            Vector2::new(4, 1),
            Vector2::new(-1, -109) % Vector2::new(5, 10)
        )
    }

    #[test]
    fn eq_vec2_both() {
        assert_eq!(Vector2::new(5, 4), Vector2::new(5, 4))
    }

    #[test]
    fn eq_vec2_only_one() {
        assert_ne!(Vector2::new(5, 2), Vector2::new(5, 4))
    }

    #[test]
    fn eq_vec2_neither() {
        assert_ne!(Vector2::new(17, 2), Vector2::new(5, 4))
    }
}
