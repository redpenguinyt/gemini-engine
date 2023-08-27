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
//!         View::display_render(view.render());
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
//! 4. print the result of `View.render`
//! 5. Sleep
//!
//! In our case, we want to move our [`Point`] one unit to the right every frame, so we increase its value by one here. Next we blit the [`Point`] to the [`View`] (adding it to the [`View`]'s internal canvas) and render. Rendering will display the view in the terminal (make sure your terminal is large enough to fit the whole image!). The last line of our code sleeps for `1/FPS` seconds. We pass None in place of what would normally be a Some(Duration) type, displaying the amount of time it took to blit and render everything so that [`gameloop::sleep_fps`](crate::gameloop::sleep_fps) can accomodate for the time taken to render. Since this example program is quite simple, we've just passed None. You can see how best to write a gameloop in the [`gameloop`](crate::gameloop) documentation.
//!
//! There you have it! You've written your first program with Gemini! As of me writing this now it's still very much a work in progress, so any feedback or issue requests would be appreciated :)

pub mod view;
use view::utils::{self, BlitCache};
use view::{ColChar, Modifier, ViewElement};
pub use view::{Vec2D, View};

/// A `PixelContainer` only has a [`pixels`](PixelContainer::pixels) property, which gets returned directly to the View during blit
pub struct PixelContainer {
    pub pixels: Vec<(Vec2D, ColChar)>,
}

impl PixelContainer {
    pub fn new() -> Self {
        Self { pixels: vec![] }
    }

    pub fn push(&mut self, pixel: (Vec2D, ColChar)) {
        self.pixels.push(pixel);
    }

    pub fn append(&mut self, pixels: &mut Vec<(Vec2D, ColChar)>) {
        self.pixels.append(pixels);
    }
}

impl ViewElement for PixelContainer {
    fn active_pixels(&self) -> Vec<(Vec2D, ColChar)> {
        self.pixels.clone()
    }
}

/// The `Point` holds a single [`Vec2D`], the coordinates at which it is printed when blit to a [`View`]
pub struct Point {
    pub pos: Vec2D,
    pub fill_char: ColChar,
    _private: (),
}

impl Point {
    pub fn new(pos: Vec2D, fill_char: ColChar) -> Self {
        Self {
            pos,
            fill_char,
            _private: (),
        }
    }
}

impl ViewElement for Point {
    fn active_pixels(&self) -> Vec<(Vec2D, ColChar)> {
        vec![(self.pos, self.fill_char)]
    }
}

/// The `Line` takes two [`Vec2D`]s and returns a line between those vertices when blit to a [`View`]
pub struct Line {
    pub pos0: Vec2D,
    pub pos1: Vec2D,
    pub fill_char: ColChar,
    cache: BlitCache<Vec2D>,
}

impl Line {
    pub fn new(pos0: Vec2D, pos1: Vec2D, fill_char: ColChar) -> Self {
        Line {
            pos0,
            pos1,
            fill_char,
            cache: BlitCache::DEFAULT,
        }
    }

    /// Generate a [`BlitCache`] if you intend for the line to not move across multiple frames. If you use this, you MUST call generate_cache if the line does move in the future. This function will not generate a new cache if the previously generated cache is still valid
    pub fn generate_cache(&mut self) {
        if !self.cache.is_cache_valid(&vec![self.pos0, self.pos1]) {
            let points = Self::draw(self.pos0, self.pos1);

            self.cache = BlitCache::new(vec![self.pos0, self.pos1], points);
        }
    }

    /// Draw a line using Bresenham's line algorithm. Returns a list of the pixels to print to
    pub fn draw(pos0: Vec2D, pos1: Vec2D) -> Vec<Vec2D> {
        // Use Bresenham's line algorithm to generate active pixels at rendertime
        let mut points: Vec<Vec2D> = Vec::new();

        let (mut x, mut y) = pos0.as_tuple();
        let (x1, y1) = pos1.as_tuple();

        let dx = (x1 - x).abs();
        let sx = if x < x1 { 1 } else { -1 };
        let dy = -(y1 - y).abs();
        let sy = if y < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            let pixel = Vec2D::new(x, y);
            points.push(pixel);
            let e2 = error * 2;
            if e2 >= dy {
                if x == x1 {
                    break;
                };
                error += dy;
                x += sx;
            };
            if e2 <= dx {
                if y == y1 {
                    break;
                };
                error += dx;
                y += sy;
            };
        }

        points
    }
}

impl ViewElement for Line {
    fn active_pixels(&self) -> Vec<(Vec2D, ColChar)> {
        let cache = self.cache.dependent();
        let points = match cache {
            Some(c) => c,
            None => Self::draw(self.pos0, self.pos1),
        };

        utils::points_to_pixels(points, self.fill_char)
    }
}

/// The `Triangle` takes three [`Vec2D`]s and returns a triangle with those vertices when blit to a [`View`]
pub struct Triangle {
    pub pos0: Vec2D,
    pub pos1: Vec2D,
    pub pos2: Vec2D,
    pub fill_char: ColChar,
    cache: BlitCache<Vec2D>,
}

impl Triangle {
    pub fn new(pos0: Vec2D, pos1: Vec2D, pos2: Vec2D, fill_char: ColChar) -> Self {
        Triangle {
            pos0,
            pos1,
            pos2,
            fill_char: fill_char,
            cache: BlitCache::DEFAULT,
        }
    }

    /// Generate a [`BlitCache`] if you intend for the triangle to not move across multiple frames. If you use this, you MUST call generate_cache if the triangle does move in the future. This function will not generate a new cache if the previously generated cache is still valid
    pub fn generate_cache(&mut self) {
        if !self.cache.is_cache_valid(&vec![self.pos0, self.pos1]) {
            let points = Self::draw(self.corners());

            self.cache = BlitCache::new(self.corners().to_vec(), points);
        }
    }

    /// Return the triangle's points as an array
    pub fn corners(&self) -> [Vec2D; 3] {
        [self.pos0, self.pos1, self.pos2]
    }

    /// Takes three corner [`Vec2D`]s and returns the points you should plot to the screen to make a triangle
    pub fn draw(corners: [Vec2D; 3]) -> Vec<Vec2D> {
        let mut points = vec![];
        let mut corners = corners;
        corners.sort_unstable_by_key(|k| k.y);
        let (x0, y0) = corners[0].as_tuple();
        let (x1, y1) = corners[1].as_tuple();
        let (x2, y2) = corners[2].as_tuple();

        let mut x01 = utils::interpolate(y0, x0 as f64, y1, x1 as f64);
        let x12 = utils::interpolate(y1, x1 as f64, y2, x2 as f64);
        let x02 = utils::interpolate(y0, x0 as f64, y2, x2 as f64);

        x01.pop();
        let mut x012 = x01;
        x012.extend(x12);

        let m = (x012.len() as f64 / 2.0).floor() as usize;
        let (x_left, x_right) = match x02[m] < x012[m] {
            true => (x02, x012),
            false => (x012, x02),
        };

        for (i, y) in (y0..y2).enumerate() {
            for x in x_left[i]..x_right[i] {
                points.push(Vec2D::new(x as isize, y));
            }
        }

        points
    }
}

impl ViewElement for Triangle {
    fn active_pixels(&self) -> Vec<(Vec2D, ColChar)> {
        let cache = self.cache.dependent();
        let points = match cache {
            Some(c) => c,
            None => Self::draw(self.corners()),
        };

        utils::points_to_pixels(points, self.fill_char)
    }
}

/// The `Polygon` takes a vec of [`Vec2D`]s and returns a polygon with those vertices when blit to a [`View`]
pub struct Polygon {
    pub points: Vec<Vec2D>,
    pub fill_char: ColChar,
    cache: BlitCache<Vec2D>,
}

impl Polygon {
    pub fn new(points: Vec<Vec2D>, fill_char: ColChar) -> Self {
        Self {
            points,
            fill_char,
            cache: BlitCache::DEFAULT,
        }
    }

    /// Generate a [`BlitCache`] if you intend for the polygin to not move across multiple frames. If you use this, you MUST call generate_cache if the polygon does move in the future. This function will not generate a new cache if the previously generated cache is still valid
    pub fn generate_cache(&mut self) {
        if !self.cache.is_cache_valid(&self.points) {
            let points = Self::draw(self.points.clone());

            self.cache = BlitCache::new(self.points.to_vec(), points);
        }
    }

    /// Draw a polygon from points. Only supports convex polygons as of now
    pub fn draw(vertices: Vec<Vec2D>) -> Vec<Vec2D> {
        let mut points = vec![];
        for fi in 1..vertices.len() {
            points.extend(Triangle::draw([
                vertices[0],
                vertices[fi],
                vertices[(fi + 1) % vertices.len()],
            ]))
        }
        points
    }
}

impl ViewElement for Polygon {
    fn active_pixels(&self) -> Vec<(Vec2D, ColChar)> {
        let cache = self.cache.dependent();
        let points = match cache {
            Some(c) => c,
            None => Self::draw(self.points.clone()),
        };

        utils::points_to_pixels(points, self.fill_char)
    }
}

/// The `Polygon` takes a position and size, and returns a box at that position with that width and size when blit to a [`View`]
pub struct Box {
    pub pos: Vec2D,
    pub size: Vec2D,
    pub fill_char: ColChar,
    _private: (),
}

impl Box {
    pub fn new(pos: Vec2D, size: Vec2D, fill_char: ColChar) -> Self {
        Self {
            pos,
            size,
            fill_char,
            _private: (),
        }
    }
}

impl ViewElement for Box {
    fn active_pixels(&self) -> Vec<(Vec2D, ColChar)> {
        let mut pixels: Vec<(Vec2D, ColChar)> = vec![];

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                pixels.push((self.pos + Vec2D { x, y }, self.fill_char))
            }
        }

        pixels
    }
}

/// A `ViewElement` that takes a multi-line string as a parameter, and can be used to put ASCII art, text and other such things on the View
pub struct Sprite {
    pub pos: Vec2D,
    pub texture: String,
    pub modifier: Modifier,
    _private: (),
}
impl Sprite {
    pub fn new(pos: Vec2D, texture: &str, modifier: Modifier) -> Self {
        let mut texture = String::from(texture);
        if texture.starts_with('\n') {
            texture.pop();
        }
        Self {
            pos,
            texture,
            modifier,
            _private: (),
        }
    }
}

impl ViewElement for Sprite {
    fn active_pixels(&self) -> Vec<(Vec2D, ColChar)> {
        let mut pixels: Vec<(Vec2D, ColChar)> = vec![];

        let lines = self.texture.split("\n");
        for (y, line) in lines.enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char != ' ' {
                    pixels.push((
                        self.pos + Vec2D::new(x as isize, y as isize),
                        ColChar {
                            fill_char: char,
                            modifier: self.modifier,
                        },
                    ));
                }
            }
        }

        pixels
    }
}
