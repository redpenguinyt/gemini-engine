//! The gameloop is one of Gemini's most important features. If you read the Quick Start guide, you'll have seen that the example there didnt have a fully written gameloop. When you begin building larger projects with Gemini, this is what your code should look like
//! ```rust,no_run
//! use gemini_engine::gameloop;
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
//!
//! You can also use the `fps_gameloop!` macro to achieve the same result. Read about how to use it in the [`fps_gameloop!`]() documentation
use std::thread::sleep;
pub use std::time::{Duration, Instant};

/// Sleep for a single frame depending on the declared FPS, while also subtracting the Duration taken to process the frame. Returns a bool value depending on whether or not the frame took longer to render than the intended fps
/// ## Example
/// ```rust,no_run
/// use gemini_engine::gameloop;
///
/// let mut frame_skip = false;
/// const FPS: u32 = 60;
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

/// You can use the `fps_gameloop!` macro to avoid writing a lot of boilerplate code. Take this block of code from a program written with gemini
/// ```rust,no_run
/// # use gemini_engine::{elements::view::{View, ColChar, Wrapping, Vec2D}, elements3d::{Viewport, Mesh3D, Transform3D, DisplayMode}, gameloop};
/// # let mut view = View::new(0, 0, ColChar::BACKGROUND);
/// # let viewport = Viewport::new(Transform3D::default(), 0.0, Vec2D::ZERO);
/// let mut cube = Mesh3D::default_cube();
///
/// let FPS = 30;
/// let mut frame_skip = false;
/// loop {
///     let now = gameloop::Instant::now();
///
///     // Logic
///     view.clear();
///     cube.transform.rotation.y += 0.1;
///
///     if frame_skip {
///         frame_skip = false;
///     } else {
///         // Rendering
///         view.blit(&viewport.render(vec![&cube], DisplayMode::Solid), Wrapping::Ignore);
///         view.display_render().unwrap();
///     }
///     let elapsed = now.elapsed();
///     frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
/// }
/// ```
/// There's a lot of very repetitive code here. That's where this macro comes in. Here is the same block of code, rewritten with `fps_gameloop!`:
/// ```rust,no_run
/// # use gemini_engine::{elements::view::{View, ColChar, Wrapping, Vec2D}, elements3d::{Viewport, Mesh3D, Transform3D, DisplayMode}, fps_gameloop};
/// # let mut view = View::new(0, 0, ColChar::BACKGROUND);
/// # let viewport = Viewport::new(Transform3D::default(), 0.0, Vec2D::ZERO);
/// let mut cube = Mesh3D::default_cube();
///
/// let FPS = 30;
/// fps_gameloop!(
///     {
///         view.clear();
///         cube.transform.rotation.y += 0.1;
///     },
///     {
///         view.blit(&viewport.render(vec![&cube], DisplayMode::Solid), Wrapping::Ignore);
///         view.display_render().unwrap();
///     },
///     FPS
/// );
/// ```
/// The code is now a lot less cluttered. This macro accepts three fragments (and an optional fourth fragment). A logic block fragment (contained inside `{}` brackets) for code that should run every single frame, a render block fragment for code related to displaying to the terminal (all plots, blits and renders) and a `u32` fragment. The fourth optional fragment is to be a function that accepts a [`Duration`] parameter representing the time taken to render everything and a `bool` parameter representing whether the last frame was skipped or not. It can be used to, say, print debug info. Here's an example:
/// ```rust,no_run
/// # use gemini_engine::{fps_gameloop, gameloop};
/// fps_gameloop!(
///     // -- other fields --
/// #   {}, {}, 0,
///     |elapsed: gameloop::Duration, frame_skip: bool| {
///         println!(
///             "Elapsed: {:.2?}µs | Frame skip: {}",
///             elapsed.as_micros(),
///             frame_skip
///         );
///     }
/// );
#[macro_export]
macro_rules! fps_gameloop {
    ($logic:block, $render:block, $fps:expr) => {
        fps_gameloop!($logic, $render, $fps, |_, _| ());
    };
    ($logic:block, $render:block, $fps:expr, $handle_elapsed:expr) => {
        use gemini_engine::gameloop::{sleep_fps, Instant};
        let mut frame_skip = false;
        loop {
            let now = Instant::now();

            $logic; // Logic

            match frame_skip {
                true => frame_skip = false,
                false => {
                    $render;
                } // Rendering
            }

            // Debug info and such
            ($handle_elapsed)(now.elapsed(), frame_skip);

            let elapsed = now.elapsed();
            frame_skip = sleep_fps($fps, Some(elapsed));
        }
    };
}
