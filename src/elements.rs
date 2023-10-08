//! Gemini's core elements module. This and the [`view`] module make up Gemini's core rendering system.
//!
//! ## Quick Start
//! Let's get started with a simple program to demonstrate how Gemini works:
//! ```rust,no_run
#![doc = include_str!("../examples/quick-start.rs")]
//! ```
//! Ok, let's go over this and see what's going on. We start by creating a [`View`] and [`Point`]. the [`View`] takes two numbers for the width and height, as well as a [`ColChar`]. The [`Point`] takes a [`Vec2D`] and a [`ColChar`].
//!
//! We use [`ColChar`] to say exactly what each pixel should look like and what colour it should be. Here we used the built in `ColChar::BACKGROUND` and `ColChar::SOLID` to keep the code simple. You can read more in the [`ColChar`] documentation.
//!
//! At its heart, [`Vec2D`] is just a pair of `isize` integers for defining things such as position, size and movement. We used it here to define the [`Point`]'s starting position, before the game loop.
//!
//! Now that we've got initialisation out of the way, let's get on to the juicy part: the main loop. In Gemini the main loop always goes as follows:
//! 1. Clear the [`View`]
//! 2. Work through any logic you might have (moving things around, taking inputs etc.)
//! 3. Blit all the [`ViewElement`]s to the screen
//! 4. print the result of `View.display_render`
//! 5. Sleep
//!
//! In our case, we want to move our [`Point`] one unit to the right every frame, so we increase its value by one here. Next we blit the [`Point`] to the [`View`] (adding it to the [`View`]'s internal canvas) and render. Rendering will display the view in the terminal (make sure your terminal is large enough to fit the whole image!). The last line of our code sleeps for `1/FPS` seconds. We pass None in place of what would normally be a Some(Duration) type, displaying the amount of time it took to blit and render everything so that [`gameloop::sleep_fps`](crate::gameloop::sleep_fps) can accomodate for the time taken to render. Since this example program is quite simple, we've just passed None. You can see how best to write a gameloop in the [`gameloop`](crate::gameloop) documentation.
//!
//! There you have it! You've written your first program with Gemini! As of me writing this now it's still very much a work in progress, so any feedback or issue requests would be appreciated :)

pub mod ascii;
pub use ascii::{Sprite, Text};

pub mod containers;
pub use containers::{PixelContainer, VisibilityToggle};

pub mod geometry;
pub use geometry::{Line, Polygon, Rect, Triangle};

pub mod view;
pub use view::{Point, Vec2D, View};

use view::utils;
use view::{ColChar, ViewElement};
