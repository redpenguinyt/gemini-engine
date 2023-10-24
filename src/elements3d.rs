//! Gemini's implementation of 3D rendering. Experimental
//!
//! ## A Simple 3D Scene
//! Let's write a simple example program to print a spinning cube:
//! ```rust,no_run
//! use gemini_engine::elements::{
//!     view::{View, ColChar, Wrapping},
//!     Vec2D,
//! };
//! use gemini_engine::elements3d::{DisplayMode, Mesh3D, Vec3D, Viewport, Transform3D};
//! use gemini_engine::gameloop;
//!
//! const FPS: f32 = 20.0;
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
//!         view.center(),
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
//!                 view.blit(
//!                     &viewport.render(vec![&cube], DisplayMode::Solid),
//!                     Wrapping::Ignore
//!                 );
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
//! In this scenario, we create a [`View`](crate::elements::view::View) of width 350 and height 90 (you may have to zoom out and expand your terminal to fit the whole image), a [`Viewport`] with a transform of rotation 0.5 radians and translation 5 units away from the centre, our desired FOV and origin point (the centre of the view we're printing to) in the middle of the [`View`](crate::elements::view::View) and a single default cube, which is 2 units tall, wide and long and is placed directly in the middle of the scene.
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
//! # use gemini_engine::elements::{view::{View, ColChar, Wrapping}, Vec2D};
//! # use gemini_engine::elements3d::{Viewport, Mesh3D, DisplayMode, Transform3D};
//! # const FOV: f64 = 5000.0;
//! # let mut view = View::new(350, 90, ColChar::BACKGROUND);
//! # let viewport = Viewport::new(
//! #     Transform3D::DEFAULT,
//! #     FOV,
//! #     view.size(),
//! # );
//! # let cube = Mesh3D::default_cube();
//! view.blit(&viewport.render(vec![&cube], DisplayMode::Solid), Wrapping::Ignore);
//! view.display_render().unwrap();
//! ```
//!
//! This part of the code renders all the 3d stuff to the [`View`](crate::elements::view::View) and blits it to the view before rendering as usual. [`Viewport.render()`](Viewport) takes a list of all the objects we want to render and a [`DisplayMode`] enum (more info in the [`DisplayMode`] documentation).

pub mod view3d;
pub use view3d::{DisplayMode, Face, Light, Transform3D, Vec3D, ViewElement3D, Viewport};

mod mesh3d;
pub use mesh3d::Mesh3D;

mod grid;
pub use grid::Grid3D;
