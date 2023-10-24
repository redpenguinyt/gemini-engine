//! The gameloop is one of Gemini's most important features. If you read the Quick Start guide, you'll have seen that the example there didnt have a fully written gameloop. When you begin building larger projects with Gemini, this is what your code should look like
//! ```rust,no_run
//! use gemini_engine::gameloop;
//!
//! const FPS: f32 = 30.0;
//!
//! fn main() {
//!     // --initialisation--
//!     let mut frame_skip = false;
//!
//!     loop {
//!         let now = gameloop::Instant::now();
//!         // --clearing views and all necessary logic--
//!
//!         if frame_skip {
//!             frame_skip = false
//!         } else {
//!             // --all blitting and rendering goes here along with any visual logic--
//!         }
//!
//!         let elapsed = now.elapsed();
//!         frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
//!     }
//! }
//! ```
//! Writing your code like this ensures that it wont affect the game's intentional speed too much, and also makes it easy for you to benchmark your game's speed with something like `println!("Elapsed: {:.2?}µs", elapsed.as_micros());` after `let elapsed`.
//!
//! You can use the `fps_gameloop!` macro to achieve the same result. Read about how to use it in the [`fps_gameloop!`](crate::fps_gameloop) documentation

pub use std::time::{Duration, Instant};

mod sleep_fps;
pub use sleep_fps::sleep_fps;

pub mod with_root;
pub use with_root::MainLoopRoot;

mod macros;
