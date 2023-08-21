pub mod view;
use view::utils;
use view::{ColChar, Modifier, ViewElement};
pub use view::{Vec2D, View};

/// The `Point` is the most basic object to implement the `ViewElement` trait
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

/// The `Line` is used to draw a line between two points
pub struct Line {
    pub pos0: Vec2D,
    pub pos1: Vec2D,
    pub fill_char: ColChar,
    _cache: (Vec2D, Vec2D, Vec<Vec2D>),
}

impl Line {
    pub fn new(pos0: Vec2D, pos1: Vec2D, fill_char: ColChar) -> Self {
        Line {
            pos0,
            pos1,
            fill_char,
            _cache: (Vec2D::ZERO, Vec2D::ZERO, vec![Vec2D::ZERO]),
        }
    }

    /// Generate a cache if you intend for the line to not move across multiple frames. If you use this, you MUST call generate_cache if the line does move in the future. This function will not generate a new cache if the previous cache has the same start and points
    pub fn generate_cache(&mut self) {
        if (self._cache.0 != self.pos0) | (self._cache.1 != self.pos1) {
            let points = Self::draw(self.pos0, self.pos1);

            self._cache = (self.pos0, self.pos1, points);
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
        let points: Vec<Vec2D>;
        if self._cache.2 != vec![Vec2D::ZERO] {
            // if the cache has been used...
            points = self._cache.2.clone(); //  use the cache
        } else {
            points = Self::draw(self.pos0, self.pos1); // otherwise draw a line from scratch
        }

        // add the
        utils::points_to_pixels(points, self.fill_char)
    }
}

/// The `Triangle` takes three `Vec2D`s and results in a triangle when blit to a `View`
pub struct Triangle {
    pub pos0: Vec2D,
    pub pos1: Vec2D,
    pub pos2: Vec2D,
    pub fill_char: ColChar,
    _private: (),
}

impl Triangle {
    pub fn new(pos0: Vec2D, pos1: Vec2D, pos2: Vec2D, fill_char: ColChar) -> Self {
        Triangle {
            pos0,
            pos1,
            pos2,
            fill_char: fill_char,
            _private: (),
        }
    }

    /// Return the triangle's points as an array
    pub fn points(&self) -> Vec<Vec2D> {
        vec![self.pos0, self.pos1, self.pos2]
    }
}

impl ViewElement for Triangle {
    fn active_pixels(&self) -> Vec<(Vec2D, ColChar)> {
        let mut points = vec![];

        let mut corners: Vec<Vec2D> = self.points();
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

        utils::points_to_pixels(points, self.fill_char)
    }
}

/// The `Box` has a position and size, with the position corresponding to its top-left corner
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
