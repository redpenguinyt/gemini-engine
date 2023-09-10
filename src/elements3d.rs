//! Gemini's implementation of 3D rendering. Experimental
//!
//! ## A Simple 3D Scene
//! Let's write a simple example program to print a spinning cube:
//! ```rust,no_run
//! use gemini_engine::elements::{
//! view::ColChar,
//! Vec2D, View,
//! };
//! use gemini_engine::elements3d::{DisplayMode, Mesh3D, Vec3D, Viewport, Transform3D};
//! use gemini_engine::gameloop;
//!
//! const FPS: u32 = 20;
//! const FOV: f64 = 95.0;
//!
//! fn main() {
//!     let mut frame_skip = false;
//!     let mut view = View::new(350, 90, ColChar::BACKGROUND);
//!
//!     let mut viewport = Viewport::new(
//!         Transform3D::new_tr(
//!             Vec3D::new(0.0, 0.0, 5.0),
//!             Vec3D::new(-0.5, 0.0, 0.0)
//!         ),
//!         FOV,
//!         Vec2D::new((view.width / 2) as isize, (view.height / 2) as isize),
//!     );
//!
//!     let cube = Mesh3D::default_cube();
//!
//!     loop {
//!         let now = gameloop::Instant::now();
//!         view.clear();
//!
//!         viewport.transform.rotation.y -= 0.05;
//!
//!         match frame_skip {
//!             true => frame_skip = false,
//!             false => {
//!                 viewport.blit_to(&mut view, vec![&cube], DisplayMode::Solid);
//!                 view.display_render().unwrap();
//!             }
//!         }
//!
//!         let elapsed = now.elapsed();
//!         println!(
//!             "Elapsed: {:.2?}µs | Frame skip: {}",
//!             elapsed.as_micros(),
//!             frame_skip
//!         );
//!
//!         frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
//!     }
//! }
//! ```
//! There is a lot of code here, but since the main loop is based off of the [`gameloop`](crate::gameloop) principle (Go to the [`gameloop`](crate::gameloop) documentation page to learn more), we'll only focus on the parts that are different from the [`gameloop`](crate::gameloop) example:
//!
//! ### Initialisation
//! ```rust,no_run
//! # use gemini_engine::elements::{View, Vec2D, view::ColChar};
//! # use gemini_engine::elements3d::{Viewport, Mesh3D, Transform3D};
//! # const FOV: f64 = 95.0;
//! let mut view = View::new(350, 90, ColChar::BACKGROUND);
//!
//! let mut viewport = Viewport::new(
//!     Transform3D::DEFAULT,
//!     FOV,
//!     view.size(),
//! );
//!
//! let cube = Mesh3D::default_cube();
//! ```
//! `main()` begins with the creation of all the necessary objects to render 3D images:
//! 1. [`View`](crate::elements::view::View) to handle the canvas and printing to the screen
//! 2. [`Viewport`] to handle converting 3d objects to 2d images, as well as acting like the scene's camera
//! 3. The actual objects you intend to use in the scene, all of which should implement the [`ViewElement3D`] trait
//!
//! In this scenario, we create a [`View`](crate::elements::view::View) of width 350 and height 90 (you may have to zoom out and expand your terminal to fit the whole image), a [`Viewport`] with a transform of rotation 0.5 radians and translation 5 units away from the centre, our desired FOV and origin point (the centre of t) in the middle of the [`View`](crate::elements::view::View) and a single default cube, which is 2 units tall, wide and long and is placed directly in the middle of the scene.
//!
//! ### Gameloop process logic
//! ```rust,no_run
//! # use gemini_engine::elements::{View, Vec2D, view::ColChar};
//! # use gemini_engine::elements3d::{Viewport, Transform3D};
//! # const FOV: f64 = 5000.0;
//! # let view = View::new(350, 90, ColChar::BACKGROUND);
//! # let mut viewport = Viewport::new(
//! #     Transform3D::DEFAULT,
//! #     FOV,
//! #     view.size(),
//! # );
//! viewport.transform.rotation.y -= 0.05;
//! ```
//!
//! This part of the code is where we would put all our physics, collisions, events etc. code, but in this case the only thing we do is rotate the cube 0.05 radians anticlockwise.
//!
//! ### Blitting/Rendering
//! ```rust,no_run
//! # use gemini_engine::elements::{View, Vec2D, view::ColChar};
//! # use gemini_engine::elements3d::{Viewport, Mesh3D, DisplayMode, Transform3D};
//! # const FOV: f64 = 5000.0;
//! # let mut view = View::new(350, 90, ColChar::BACKGROUND);
//! # let viewport = Viewport::new(
//! #     Transform3D::DEFAULT,
//! #     FOV,
//! #     view.size(),
//! # );
//! # let cube = Mesh3D::default_cube();
//! viewport.blit_to(&mut view, vec![&cube], DisplayMode::Solid);
//! view.display_render().unwrap();
//! ```
//!
//! This part of the code blits all the 3d stuff to the [`View`](crate::elements::view::View) before rendering as usual. [`Viewport.blit_to()`](Viewport#blit_to) takes a mutable reference to the view, a list of all the objects we want to render and a [`DisplayMode`] enum (more info in the [`DisplayMode`] documentation).

use crate::elements::view::{ColChar, Modifier, Vec2D};
pub mod view3d;
pub use view3d::{DisplayMode, Face, Transform3D, Vec3D, ViewElement3D, Viewport};

/// The struct for a Mesh3D object, containing a position, rotation, collection of vertices and collection of [`Face`]s with indices to the vertex collection.
#[derive(Debug)]
pub struct Mesh3D {
    pub transform: Transform3D,
    pub vertices: Vec<Vec3D>,
    pub faces: Vec<Face>,
}

impl Mesh3D {
    /// The gemini_engine equivalent of Blender's default cube. Has side lengths of 2
    pub fn default_cube() -> Self {
        Self::new(
            Transform3D::DEFAULT,
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

    /// A gimbal to help you orient in gemini_engine's 3D space. The orientation is as follows (from the default [`Viewport`])
    /// - X (red) increases as you move to the right
    /// - Y (green) increases as you move up
    /// - Z (blue) increases as you move away from the viewport
    ///
    /// Think of it like Blender's axes but with Y and Z swapped.
    /// This Mesh does not render in `DisplayMode::SOLID` (see [`DisplayMode`] documentation)
    pub fn gimbal() -> Self {
        Self::new(
            Transform3D::DEFAULT,
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

    pub fn new(transform: Transform3D, vertices: Vec<Vec3D>, faces: Vec<Face>) -> Self {
        Self {
            transform,
            vertices,
            faces,
        }
    }
}

impl Clone for Mesh3D {
    fn clone(&self) -> Self {
        Self {
            transform: self.transform,
            vertices: self.vertices.clone(),
            faces: self.faces.clone(),
        }
    }
}

impl ViewElement3D for Mesh3D {
    fn get_transform(&self) -> Transform3D {
        self.transform
    }
    fn get_vertices(&self) -> Vec<Vec3D> {
        self.vertices.clone()
    }
    fn get_faces(&self) -> Vec<Face> {
        self.faces.clone()
    }
    fn vertices_on_screen(&self, viewport: &Viewport) -> Vec<(Vec2D, f64)> {
        let mut screen_vertices = vec![];
        for vertex in &self.vertices {
            // (viewport.transform * self.transform) *
            let transformed = (viewport.transform * self.transform) * *vertex;

            // println!("{vertex}, {transformed}");

            let screen_coordinates = viewport.spatial_to_screen(transformed);
            screen_vertices.push((screen_coordinates, transformed.z));
        }

        // println!("{:#?}", screen_vertices);

        // panic!("intentional");

        screen_vertices
    }
}
