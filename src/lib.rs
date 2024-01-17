#![doc = include_str!("../README.md")]
//!
//! Go to [`elements`] for a quick start guide.
//!
//! ## Crate Structure
//! This library is made up of three main crates:
//! - [`gameloop`], which handles the gameloop. See the [`gameloop`] documentation to see how to structure the usual Gemini project.
//! - [`elements`], which handles the printing of various objects to a [`View`](elements::View), the central object in a Gemini project.
//! - [`elements3d`], which handles everything 3D-related. Objects that [`elements3d`] converts to a 2d object will then be printed to the screen by a [`View`](elements::View)

#![warn(missing_docs)] // For making sure everything's documented
#![warn(rustdoc::missing_doc_code_examples)]

#[macro_use]
mod utils;

pub mod elements;
pub mod elements3d;
pub mod gameloop;
