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
            let points = Self::draw(self.pos0, self.pos1);

            self._cache = (self.pos0, self.pos1, points);
        }
    }

    /// draw line using Bresenham's line algorithm
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
    fn active_pixels(&self) -> Vec<(Vec2D, char)> {
        let points: Vec<Vec2D>;
        if self._cache.2 != vec![Vec2D::ZERO] {
            // if the cache has been used...
            points = self._cache.2.clone(); //  use the cache
        } else {
            points = Self::draw(self.pos0, self.pos1); // otherwise draw a line from scratch
        }

        // add the
        points_to_pixels(points, self.fill_char)
    }
}

// A Polygon holds an arbitrary number of Vec2D values that mark each vertex of the drawn polygon
pub struct Triangle {
    pub pos0: Vec2D,
    pub pos1: Vec2D,
    pub pos2: Vec2D,
    pub fill_char: char,
    _private: (),
}

impl Triangle {
    pub fn new(pos0: Vec2D, pos1: Vec2D, pos2: Vec2D, fill_char: char) -> Self {
        Triangle {
            pos0,
            pos1,
            pos2,
            fill_char: fill_char,
            _private: (),
        }
    }

    /// return triangle's points as an array
    pub fn points(&self) -> [Vec2D; 3] {
        [self.pos0, self.pos1, self.pos2]
    }
}

impl ViewElement for Triangle {
    fn active_pixels(&self) -> Vec<(Vec2D, char)> {
        // create triangle borders
        let mut border_points: Vec<Vec2D> = vec![];

        border_points.append(&mut Line::draw(self.pos0, self.pos1));
        border_points.append(&mut Line::draw(self.pos1, self.pos2));
        border_points.append(&mut Line::draw(self.pos2, self.pos0));

        // begin creating set of final points
        let mut points: Vec<Vec2D> = vec![];

        let corners = self.points();
        let min_max_x = (
            corners
                .iter()
                .min_by_key(|k| k.x)
                .expect("vector is (somehow) empty"),
            corners
                .iter()
                .max_by_key(|k| k.x)
                .expect("vector is (somehow) empty"),
        );
        let min_max_y = (
            corners
                .iter()
                .min_by_key(|k| k.y)
                .expect("vector is (somehow) empty"),
            corners
                .iter()
                .max_by_key(|k| k.y)
                .expect("vector is (somehow) empty"),
        );

        for x in min_max_x.0.x..(min_max_x.1.x + 1) {
            let mut fill = false;
            let mut first_found_point = None;
            let mut filled_points = vec![];
            for y in min_max_y.0.y..(min_max_y.1.y + 1) {
                let point = Vec2D::new(x, y);

                if border_points.contains(&point) {
                    fill = !fill;
                    if !fill {
                        filled_points.push(Vec2D::new(x, y));
                        break;
                    }
                }
                if fill {
                    if first_found_point.is_none() {
                        first_found_point = Some(point);
                    }
                    filled_points.push(point);
                }
            }

            match fill {
                // everything went correctly
                false => points.append(&mut filled_points),
                // the triangle was never closed
                true => match first_found_point {
                    Some(p) => points.push(p),
                    None => ()
                }
            }
        }

        points_to_pixels(points, self.fill_char)
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
            _private: (),
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
