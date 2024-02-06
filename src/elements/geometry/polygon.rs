use super::Triangle;
use crate::elements::view::{utils, ColChar, Pixel, Vec2D, ViewElement};

fn is_left_turn(p0: Vec2D, p1: Vec2D, p2: Vec2D) -> bool {
    let v1 = p1 - p0;
    let v2 = p2 - p0;
    v1.cross(v2) > 0
}

fn is_ear(vertex: Vec2D, prev_vertex: Vec2D, next_vertex: Vec2D, polygon: &[Vec2D]) -> bool {
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];
        let p3 = polygon[(i + 2) % polygon.len()];

        if p1 != vertex
            && p2 != vertex
            && p3 != vertex
            && is_left_turn(vertex, p1, p2)
            && is_left_turn(vertex, p2, p3)
            && is_left_turn(vertex, p3, p1)
        {
            let triangle_area = (p1 - p2).cross(p3 - p2).abs();
            if triangle_area > 0 {
                let p = (vertex - prev_vertex).cross(next_vertex - prev_vertex);
                return (p1 - prev_vertex).cross(p1 - next_vertex) > 0 && p > 0;
            }
        }
    }
    false
}

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

    /// Split a polygon up into triangles using the ear cutting algorithm. Returns a vec of coordinate sets for each triangle
    #[must_use]
    pub fn triangulate(vertices: &[Vec2D]) -> Vec<[Vec2D; 3]> {
        let mut triangles = Vec::new();
        let n = vertices.len();

        if n < 3 {
            return triangles;
        }

        let mut remaining_vertices = vertices.to_vec();

        while remaining_vertices.len() > 3 {
            let mut ear_index = 0;
            for i in 0..remaining_vertices.len() {
                let prev_index = (i + remaining_vertices.len() - 1) % remaining_vertices.len();
                let next_index = (i + 1) % remaining_vertices.len();

                let vertex = remaining_vertices[i];
                let prev_vertex = remaining_vertices[prev_index];
                let next_vertex = remaining_vertices[next_index];

                if is_ear(vertex, prev_vertex, next_vertex, &remaining_vertices) {
                    ear_index = i;
                    break;
                }
            }

            let ear_vertex = remaining_vertices[ear_index];
            let prev_index = (ear_index + remaining_vertices.len() - 1) % remaining_vertices.len();
            let next_index = (ear_index + 1) % remaining_vertices.len();
            let prev_vertex = remaining_vertices[prev_index];
            let next_vertex = remaining_vertices[next_index];

            triangles.push([prev_vertex, ear_vertex, next_vertex]);
            remaining_vertices.remove(ear_index);
        }

        triangles.push([
            remaining_vertices[0],
            remaining_vertices[1],
            remaining_vertices[2],
        ]);

        triangles
    }

    /// Draw a polygon from points. Supports convex and concave polygons
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
