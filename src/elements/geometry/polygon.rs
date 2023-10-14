use super::Triangle;
use crate::elements::view::{utils, ColChar, Point, Vec2D, ViewElement};

/// The `Polygon` takes a vec of [`Vec2D`]s and returns a polygon with those vertices when blit to a [`View`](super::super::View)
pub struct Polygon {
    pub points: Vec<Vec2D>,
    pub fill_char: ColChar,
}

impl Polygon {
    pub const fn new(points: Vec<Vec2D>, fill_char: ColChar) -> Self {
        Self { points, fill_char }
    }

    /// Split a polygon up into triangles. Returns a vec of coordinate sets for each triangle
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
