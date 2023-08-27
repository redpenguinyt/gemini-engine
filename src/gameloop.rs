//! The gameloop is one of Gemini's most important features. If you read the Quick Start guide, you'll have seen that the example there didnt have a fully written gameloop. When you begin building larger projects with Gemini, this is what your code should look like
//! ```rust,ignore
//! use gemini::gameloop;
//!
//! const FPS: u32 = 30;
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
use std::thread::sleep;
pub use std::time::{Duration, Instant};

/// Sleep for a single frame depending on the declared FPS, while also subtracting the Duration taken to process the frame. Returns a bool value depending on whether or not the frame took longer to render than the intended fps
/// ## Example
/// ```rust,ignore
/// use gemini::gameloop;
///
/// let mut frame_skip = false;
/// let FPS = 60;
/// loop {
///     let now = gameloop::Instant::now();
///
///     // all code here will run at 60 FPS
///
///     if frame_skip {
///         frame_skip = false;
///     } else {
///         // calculations and rendering
///     }
///
///     frame_skip = gameloop::sleep_fps(FPS, Some(now.elapsed()));
/// }
pub fn sleep_fps(fps: u32, elapsed: Option<Duration>) -> bool {
    let elapsed = elapsed.unwrap_or(Duration::ZERO);
    let frame_length = Duration::from_secs_f32(1.0 / fps as f32);
    if frame_length > elapsed {
        sleep(frame_length - elapsed);
        return false;
    } else {
        return true;
    }
}
