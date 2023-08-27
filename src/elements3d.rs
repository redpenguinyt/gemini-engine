//! Gemini's implementation of 3D rendering. Experimental
//!
//! ## A Simple 3D Scene
//! Let's write a simple example program to print a spinning cube:
//! ```
//! use gemini::elements::{
//! view::ColChar,
//! Vec2D, View,
//! };
//! use gemini::elements3d::{DisplayMode, Mesh3D, Vec3D, Viewport};
//! use gemini::gameloop;
//!
//! const FPS: u32 = 20;
//! const FOV: f64 = 5000.0;
//!
//! fn main() {
//!     let mut frame_skip = false;
//!     let mut view = View::new(350, 90, ColChar::BACKGROUND);
//!
//!     let mut viewport = Viewport::new(
//!         Vec3D::new(0.0, 0.0, 250.0),
//!         Vec3D::new(-0.5, 0.0, 0.0),
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
//!         viewport.rotation.y -= 0.05;
//!
//!         match frame_skip {
//!             true => frame_skip = false,
//!             false => {
//!                 viewport.blit_to(&mut view, vec![&cube], DisplayMode::Solid);
//!                 View::display_render(view.render());
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
//! There is a lot of code here, but since it's based off of the `gameloop` principle (Go to the `gameloop` documentation page to learn more), we'll only focus on the parts that are different from the `gameloop` example:
//!
//! ### Initialisation
//! ```
//! let mut view = View::new(350, 90, ColChar::BACKGROUND);
//!
//! let mut viewport = Viewport::new(
//!     Vec3D::new(0.0, 0.0, 250.0),
//!     Vec3D::new(-0.5, 0.0, 0.0),
//!     FOV,
//!     Vec2D::new((view.width / 2) as isize, (view.height / 2) as isize),
//! );
//!
//! let cube = Mesh3D::default_cube();
//! ```
//! `main()` begins with the creation of all the necessary objects to render 3D images:
//! 1. `elements::view::View` to handle the canvas and printing to the screen
//! 2. `elements3d::view3d::Viewport` to handle converting 3d objects to 2d images, as well as acting like the scene's camera
//! 3. The actual objects you intend to use in the scene, all of which should implement the `elements3d::view3d::ViewElement3D` trait
//!
//! In this scenario, we create a `View` of width 350 and height 90 (you may have to zoom out and expand your terminal to fit the whole image), a `Viewport` with an initial position 250 units away from the centre and pivoted 0.5 radians up with an origin point in the middle of the View and a single default cube, which is 2 units tall, wide and long and is placed directly in the middle of the scene.
//!
//! ### Gameloop process logic
//! ```
//! viewport.rotation.y -= 0.05;
//! ```
//!
//! This part of the code is where we would put all our physics, collisions, events etc. code, but in this case the only thing we do is rotate the cube 0.05 radians anticlockwise.
//!
//! ### Blitting/Rendering
//! ```
//! viewport.blit_to(&mut view, vec![&cube], DisplayMode::Solid);
//! View::display_render(view.render());
//! ```
//!
//! This part of the code blits all the 3d stuff to the `View` before rendering as usual. `Viewport.blit_to()` takes a mutable reference to the view, a list of all the objects we want to render and a DisplayMode enum (more info in the `DisplayMode` documentation).

use crate::elements::view::{ColChar, Modifier, Vec2D};
pub mod view3d;
pub use view3d::{DisplayMode, Face, SpatialAxis, Vec3D, ViewElement3D, Viewport};

pub struct Mesh3D {
    pub pos: Vec3D,
    pub rotation: Vec3D,
    pub vertices: Vec<Vec3D>,
    pub faces: Vec<Face>,
}

impl Mesh3D {
    pub fn default_cube() -> Self {
        Self::new(
            Vec3D::ZERO,
            Vec3D::ZERO,
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
                Face::new(vec![2, 3, 1, 0], ColChar::new('a', Modifier::BLUE)),
                Face::new(vec![4, 5, 7, 6], ColChar::new('b', Modifier::BLUE)),
                Face::new(vec![1, 3, 7, 5], ColChar::new('c', Modifier::None)),
                Face::new(vec![4, 6, 2, 0], ColChar::new('d', Modifier::None)),
                Face::new(vec![6, 7, 3, 2], ColChar::new('e', Modifier::RED)),
                Face::new(vec![0, 1, 5, 4], ColChar::new('f', Modifier::RED)),
            ],
        )
    }

    pub fn new(pos: Vec3D, rotation: Vec3D, vertices: Vec<Vec3D>, faces: Vec<Face>) -> Self {
        Self {
            pos: pos,
            rotation: rotation,
            vertices: vertices,
            faces: faces,
        }
    }
}

impl Clone for Mesh3D {
    fn clone(&self) -> Self {
        Self {
            pos: self.pos,
            rotation: self.rotation,
            vertices: self.vertices.clone(),
            faces: self.faces.clone(),
        }
    }
}

impl ViewElement3D for Mesh3D {
    fn get_pos(&self) -> Vec3D {
        self.pos.clone()
    }
    fn get_rotation(&self) -> Vec3D {
        self.rotation.clone()
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
            let pos = vertex.global_position(&viewport, self);

            let screen_coordinates = viewport.origin + pos.spatial_to_screen(viewport.fov);
            screen_vertices.push((screen_coordinates, pos.z));
        }

        screen_vertices
    }
}
