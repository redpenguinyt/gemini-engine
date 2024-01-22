use crate::elements::view::{utils, ColChar, Pixel, Vec2D, ViewElement};

/// The `Line` takes two [`Vec2D`]s and returns a line between those vertices when blit to a [`View`](super::super::View)
pub struct Line {
    /// The start positon of the line
    pub pos0: Vec2D,
    /// The end position of the line
    pub pos1: Vec2D,
    /// The [`ColChar`] used to colour the line
    pub fill_char: ColChar,
}

impl Line {
    /// Create a new line with a start and end point and a [`ColChar`]
    #[must_use]
    pub const fn new(pos0: Vec2D, pos1: Vec2D, fill_char: ColChar) -> Self {
        Self {
            pos0,
            pos1,
            fill_char,
        }
    }

    /// Draw a line using Bresenham's line algorithm. Returns a list of the pixels to print to
    #[must_use]
    pub fn draw(pos0: Vec2D, pos1: Vec2D) -> Vec<Vec2D> {
        // Use Bresenham's line algorithm to generate active pixels at rendertime
        let mut points = Vec::new();

        let (mut x, mut y) = pos0.as_tuple();
        let (x1, y1) = pos1.as_tuple();

        let dx = (x1 - x).abs();
        let sx = if x < x1 { 1 } else { -1 };
        let dy = -(y1 - y).abs();
        let sy = if y < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            let point = Vec2D::new(x, y);
            points.push(point);
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
    fn active_pixels(&self) -> Vec<Pixel> {
        utils::points_to_pixels(self.active_points(), self.fill_char)
    }

    fn active_points(&self) -> Vec<Vec2D> {
        Self::draw(self.pos0, self.pos1)
    }
}
