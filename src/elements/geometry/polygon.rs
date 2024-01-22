use super::Triangle;
use crate::elements::view::{utils, ColChar, Pixel, Vec2D, ViewElement};

/// The `Polygon` takes a vec of [`Vec2D`]s and returns a polygon with those vertices when blit to a [`View`](super::super::View)
pub struct Polygon {
    /// The points that make up the polygon
    pub points: Vec<Vec2D>,
    /// The [`ColChar`] used to fill the polygon
    pub fill_char: ColChar,
}

impl Polygon {
    /// Create a new polygon
    #[must_use]
    pub const fn new(points: Vec<Vec2D>, fill_char: ColChar) -> Self {
        Self { points, fill_char }
    }

    /// Split a polygon up into triangles. Returns a vec of coordinate sets for each triangle
    #[must_use]
    pub fn triangulate(vertices: &[Vec2D]) -> Vec<[Vec2D; 3]> {
        // TODO: add an actual triangulation algorithm here!
        let mut points = vec![];
        for fi in 1..(vertices.len() - 1) {
            points.push([vertices[0], vertices[fi], vertices[fi + 1]]);
        }
        points
    }

    /// Draw a polygon from points. Only supports convex polygons as of now
    #[must_use]
    pub fn draw(vertices: &[Vec2D]) -> Vec<Vec2D> {
        Self::triangulate(vertices)
            .iter()
            .flat_map(|corners| Triangle::draw(*corners))
            .collect()
    }
}

impl ViewElement for Polygon {
    fn active_pixels(&self) -> Vec<Pixel> {
        utils::points_to_pixels(&self.active_points(), self.fill_char)
    }

    fn active_points(&self) -> Vec<Vec2D> {
        Self::draw(&self.points)
    }
}
