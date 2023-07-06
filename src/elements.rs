mod view; pub use view::{Vec2D, View, ViewElement};

/// The most basic object to implement the `ViewElement` trait
pub struct Point {
    pub pos: Vec2D,
    pub fill_char: char,
	_private: ()
}

impl Point {
    pub fn new(pos: Vec2D, fill_char: char) -> Self {
		Self { pos, fill_char, _private: () }
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
	_private: ()
}

impl Line {
	pub fn new(pos0: Vec2D, pos1: Vec2D, fill_char: char) -> Self {
		Line { pos0, pos1, fill_char, _private: () }
	}
}

impl ViewElement for Line {
	fn active_pixels(&self) -> Vec<(Vec2D, char)> {
		// Use Bresenham's line algorithm to generate active pixels at rendertime
        let mut pixels: Vec<(Vec2D, char)> = Vec::new();

        let (mut x, mut y) = self.pos0.as_tuple();
        let (x1, y1) = self.pos1.as_tuple();

        let dx = (x1 - x).abs();
        let sx = if x < x1 { 1 } else { -1 };
        let dy = -(y1 - y).abs();
        let sy = if y < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            let pixel = Vec2D { x, y };
            pixels.push(
                (pixel, self.fill_char)
            );
            let e2 = error * 2;
            if e2 >= dy {
                if x == x1 { break };
                error += dy;
                x += sx;
            };
            if e2 <= dx {
                if y == y1 { break };
                error += dx;
                y += sy;
            };
        }

        pixels
    }
}

// A Box holds two `Vec2D` values that correspond to its position in the view and size respectively
pub struct Box {
    pub pos: Vec2D,
    pub size: Vec2D,
    pub fill_char: char,
	_private: ()
}

impl Box {
    pub fn new(pos: Vec2D, size: Vec2D, fill_char: char) -> Self {
		Self { pos, size, fill_char, _private: () }
	}
}

impl ViewElement for Box {
    fn active_pixels(&self) -> Vec<(Vec2D, char)> {
        let mut pixels: Vec<(Vec2D, char)> = vec![];

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                pixels.push((
                    self.pos + Vec2D { x, y },
                    self.fill_char
                ))
            }
        }

        pixels
    }
}