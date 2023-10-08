//! This module contains basic geometry structs that implement [`ViewElement`], such as [`Line`] or [`Triangle`]

use super::view::{utils, ColChar, Point, Vec2D, ViewElement};

/// The `Line` takes two [`Vec2D`]s and returns a line between those vertices when blit to a [`View`](super::View)
pub struct Line {
    pub pos0: Vec2D,
    pub pos1: Vec2D,
    pub fill_char: ColChar,
}

impl Line {
    pub const fn new(pos0: Vec2D, pos1: Vec2D, fill_char: ColChar) -> Self {
        Line {
            pos0,
            pos1,
            fill_char,
        }
    }

    /// Draw a line using Bresenham's line algorithm. Returns a list of the pixels to print to
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
    fn active_pixels(&self) -> Vec<Point> {
        utils::points_to_pixels(Self::draw(self.pos0, self.pos1), self.fill_char)
    }
}

/// The `Triangle` takes three [`Vec2D`]s and returns a triangle with those vertices when blit to a [`View`](super::View)
pub struct Triangle {
    pub pos0: Vec2D,
    pub pos1: Vec2D,
    pub pos2: Vec2D,
    pub fill_char: ColChar,
}

impl Triangle {
    pub const fn new(pos0: Vec2D, pos1: Vec2D, pos2: Vec2D, fill_char: ColChar) -> Self {
        Triangle {
            pos0,
            pos1,
            pos2,
            fill_char,
        }
    }

    /// Takes the corners of the triangle as an array rather than as separate parameters
    pub const fn with_array(points: &[Vec2D], fill_char: ColChar) -> Self {
        if points.len() != 3 {
            panic!(
                "points parameter should have exactly 3 items, one for each point of the triangle"
            )
        }
        Self::new(points[0], points[1], points[2], fill_char)
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
                points.push(Vec2D::new(x, y));
            }
        }

        points
    }
}

impl ViewElement for Triangle {
    fn active_pixels(&self) -> Vec<Point> {
        utils::points_to_pixels(Self::draw(self.corners()), self.fill_char)
    }
}

/// The `Polygon` takes a vec of [`Vec2D`]s and returns a polygon with those vertices when blit to a [`View`](super::View)
pub struct Polygon {
    pub points: Vec<Vec2D>,
    pub fill_char: ColChar,
}

impl Polygon {
    pub const fn new(points: Vec<Vec2D>, fill_char: ColChar) -> Self {
        Self { points, fill_char }
    }

    /// Split a polygon up into triangles. Returns a vec of coordinate sets for said triangles
    pub fn triangulate(vertices: &[Vec2D]) -> Vec<[Vec2D; 3]> {
        let mut points = vec![];
        for fi in 1..(vertices.len() - 1) {
            points.push([vertices[0], vertices[fi], vertices[fi + 1]])
        }
        points
    }

    /// Draw a polygon from points. Only supports convex polygons as of now
    pub fn draw(vertices: &[Vec2D]) -> Vec<Vec2D> {
        Self::triangulate(vertices)
            .iter()
            .flat_map(|corners| Triangle::draw(*corners))
            .collect()
    }
}

impl ViewElement for Polygon {
    fn active_pixels(&self) -> Vec<Point> {
        utils::points_to_pixels(Self::draw(&self.points), self.fill_char)
    }
}

/// The `Rect` takes a position and size, and returns a rectangle at that position with the given width and size when blit to a [`View`](super::View)
pub struct Rect {
    /// The position of the top-left corner of the `Rect`
    pub pos: Vec2D,
    /// The size of the `Rect`, extending from [`Rect::pos`]
    pub size: Vec2D,
    pub fill_char: ColChar,
}

impl Rect {
    pub const fn new(pos: Vec2D, size: Vec2D, fill_char: ColChar) -> Self {
        Self {
            pos,
            size,
            fill_char,
        }
    }

    /// Draw a Rectangle with a given position (representing the top-left corner) and size
    pub fn draw(pos: Vec2D, size: Vec2D) -> Vec<Vec2D> {
        (0..size.x)
            .flat_map(|x| (0..size.y).map(move |y| pos + Vec2D { x, y }))
            .collect()
    }
}

impl ViewElement for Rect {
    fn active_pixels(&self) -> Vec<Point> {
        utils::points_to_pixels(Rect::draw(self.pos, self.size), self.fill_char)
    }
}
