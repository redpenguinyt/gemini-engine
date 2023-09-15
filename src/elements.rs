//! Gemini's core elements module. This and the [`view`] module make up Gemini's core rendering pipeline.
//!
//! ## Quick Start
//! Let's get started with a simple program to demonstrate how Gemini works:
//! ```rust,no_run
//! use gemini_engine::elements::{Point, Vec2D, view::{View, ColChar, Wrapping}};
//! use gemini_engine::gameloop;
//!
//! const FPS: u32 = 30;
//!
//! fn main() {
//!     let mut view = View::new(40, 8, ColChar::BACKGROUND);
//!     let mut point = Point::new(Vec2D::new(10,5), ColChar::SOLID);
//!
//!     loop {
//!         view.clear();
//!
//!         point.pos.x += 1;
//!
//!         view.blit(&point, Wrapping::Wrap);
//!         view.display_render().unwrap();
//!
//!         gameloop::sleep_fps(FPS, None);
//!     }
//! }
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
pub mod geometry;
pub mod view;
pub use ascii::{Sprite, Text};
pub use geometry::{Line, Polygon, Rect, Triangle};
use view::utils;
use view::{ColChar, ViewElement};
pub use view::{Point, Vec2D, View};

/// A `PixelContainer` only has a [`pixels`](PixelContainer::pixels) property, which gets returned directly to the View during blit
pub struct PixelContainer {
    pub pixels: Vec<Point>,
}

impl PixelContainer {
    pub const fn new() -> Self {
        Self { pixels: vec![] }
    }

    /// Add a single pixel to the `PixelContainer`
    pub fn push(&mut self, pixel: Point) {
        self.pixels.push(pixel);
    }

    /// Moves all the pixels into the `PixelContainer`, leaving the input empty.
    pub fn append(&mut self, pixels: &mut Vec<Point>) {
        self.pixels.append(pixels);
    }

    /// Append vector of coordinates and a single [`ColChar`] for all of them.
    pub fn append_points(&mut self, points: Vec<Vec2D>, fill_char: ColChar) {
        self.append(&mut utils::points_to_pixels(points, fill_char));
    }

    /// Plot a pixel to the PixelContainer
    pub fn plot(&mut self, pos: Vec2D, c: ColChar) {
        self.push(Point::new(pos, c))
    }

    /// Blit a [`ViewElement`] to the PixelContainer.
    pub fn blit<T: ViewElement>(&mut self, element: &T) {
        let mut active_pixels = element.active_pixels();

        self.append(&mut active_pixels);
    }
}

impl From<Vec<Point>> for PixelContainer {
    fn from(pixels: Vec<Point>) -> Self {
        Self { pixels }
    }
}

impl From<Vec<(Vec2D, ColChar)>> for PixelContainer {
    fn from(pixels: Vec<(Vec2D, ColChar)>) -> Self {
        Self {
            pixels: pixels.iter().map(|x| Point::from(*x)).collect(),
        }
    }
}

impl ViewElement for PixelContainer {
    fn active_pixels(&self) -> Vec<Point> {
        self.pixels.clone()
    }
}

/// `VisibilityToggle` is a container for a `ViewElement` with a property `visible`. When blit to the view the contained element will only appear if `visible` is `true`
pub struct VisibilityToggle<T: ViewElement> {
    pub element: T,
    pub visible: bool,
}

impl<T: ViewElement> VisibilityToggle<T> {
    pub fn new(element: T, visible: bool) -> Self {
        Self { element, visible }
    }
}

impl<T: ViewElement> ViewElement for VisibilityToggle<T> {
    fn active_pixels(&self) -> Vec<Point> {
        match self.visible {
            true => self.element.active_pixels(),
            false => vec![],
        }
    }
}
