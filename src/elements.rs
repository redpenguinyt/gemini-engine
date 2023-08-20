mod view;
pub use view::{Vec2D, View, ViewElement};

fn points_to_pixels(points: Vec<Vec2D>, fill_char: char) -> Vec<(Vec2D, char)> {
    let mut pixels: Vec<(Vec2D, char)> = Vec::new();

    for point in points {
        pixels.push((point, fill_char));
    }

    pixels
}

/// The most basic object to implement the `ViewElement` trait
pub struct Point {
    pub pos: Vec2D,
    pub fill_char: char,
    _private: (),
}

impl Point {
    pub fn new(pos: Vec2D, fill_char: char) -> Self {
        Self {
            pos,
            fill_char,
            _private: (),
        }
    }
}

impl ViewElement for Point {
    fn active_pixels(&self) -> Vec<(Vec2D, char)> {
        return Vec::from([(self.pos, self.fill_char)]);
    }
}

/// A Line object holds two `Vec2D` values that signify its start and end position
pub struct Line {
    pub pos0: Vec2D,
    pub pos1: Vec2D,
    pub fill_char: char,
    _cache: (Vec2D, Vec2D, Vec<Vec2D>),
}

impl Line {
    pub fn new(pos0: Vec2D, pos1: Vec2D, fill_char: char) -> Self {
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
            let points = self.draw_line();

            self._cache = (self.pos0, self.pos1, points);
        }
    }

    fn draw_line(&self) -> Vec<Vec2D> {
        // Use Bresenham's line algorithm to generate active pixels at rendertime
        let mut points: Vec<Vec2D> = Vec::new();

        let (mut x, mut y) = self.pos0.as_tuple();
        let (x1, y1) = self.pos1.as_tuple();

        let dx = (x1 - x).abs();
        let sx = if x < x1 { 1 } else { -1 };
        let dy = -(y1 - y).abs();
        let sy = if y < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            let pixel = Vec2D { x, y };
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
    fn active_pixels(&self) -> Vec<(Vec2D, char)> {
        let points: Vec<Vec2D>;
        if self._cache.2 != vec![Vec2D::ZERO] {
            // if the cache has been used...
            points = self._cache.2.clone(); //  use the cache
        } else {
            points = self.draw_line(); // otherwise draw a line from scratch
        }

        // add the
        points_to_pixels(points, self.fill_char)
    }
}

// A Polygon holds an arbitrary number of Vec2D values that mark each vertex of the drawn polygon
pub struct Polygon {
    pub points: Vec<Vec2D>,
    pub fill_char: char,
    _private: (),
}

impl Polygon {
    pub fn new(points: Vec<Vec2D>, fill_char: char) -> Self {
        Polygon {
            points: points,
            fill_char: fill_char,
            _private: (),
        }
    }
}

impl ViewElement for Polygon {
    fn active_pixels(&self) -> Vec<(Vec2D, char)> {
        // Initializing All of the Edges
        let mut all_edges: Vec<(isize, isize, isize, f64)> = vec![];

        for i in 0..self.points.len() {
            let (x0, y0) = self.points.get(i).expect("Element was not Vec2D").as_tuple();
            let (x1, y1) = self.points.get((i + 1) % self.points.len()).expect("Element was not Vec2D").as_tuple();

            let y0_smaller = y0 < y1;
            let min_y = if y0_smaller { y0 } else { y1 };
            let max_y = if y0_smaller { y1 } else { y0 };
            let min_x = if y0_smaller { x0 } else { x1 };
            let slope = (y0 - y1) as f64 / (x0 - x1) as f64;
            all_edges.push((
                min_y,
                max_y,
                min_x,
                slope
            ))
        }

        todo!();
        // keep writing algorithm (https://www.cs.rit.edu/~icss571/filling/how_to.html)
    }
}

// A Box holds two `Vec2D` values that correspond to its position in the view and size respectively
pub struct Box {
    pub pos: Vec2D,
    pub size: Vec2D,
    pub fill_char: char,
    _private: (),
}

impl Box {
    pub fn new(pos: Vec2D, size: Vec2D, fill_char: char) -> Self {
        Self {
            pos,
            size,
            fill_char,
            _private: (),
        }
    }
}

impl ViewElement for Box {
    fn active_pixels(&self) -> Vec<(Vec2D, char)> {
        let mut pixels: Vec<(Vec2D, char)> = vec![];

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                pixels.push((self.pos + Vec2D { x, y }, self.fill_char))
            }
        }

        pixels
    }
}

pub struct Sprite {
    pub pos: Vec2D,
    pub texture: String,
    _private: (),
}
impl Sprite {
    pub fn new(pos: Vec2D, texture: &str) -> Self {
        let mut texture = String::from(texture);
        if texture.starts_with('\n') {
            texture.pop();
        }
        Self {
            pos,
            texture,
            _private: ()
        }
    }
}

impl ViewElement for Sprite {
    fn active_pixels(&self) -> Vec<(Vec2D, char)> {
        let mut pixels: Vec<(Vec2D, char)> = vec![];

        let lines = self.texture.split("\n");
        for (y, line) in lines.enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char != ' ' {
                    pixels.push((self.pos + Vec2D::new(x as isize, y as isize), char));
                }
            }
        }

        pixels
    }
}