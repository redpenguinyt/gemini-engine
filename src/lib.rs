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

#[warn(missing_docs)]
pub mod elements;
#[warn(missing_docs)]
pub mod elements3d;
#[warn(missing_docs)]
pub mod gameloop;
