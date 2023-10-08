//! A module containing various helper functions and structs

use super::{ColChar, Point, Vec2D};

/// Combine a vector of [`Vec2D`]s and a single `fill_char` into a vector of `(Vec2D, char)` tuples, ready to return for `ViewElement::active_pixels`. Useful if your [`ViewElement`](super::ViewElement) only has one fill character across all of it
pub fn points_to_pixels(points: Vec<Vec2D>, fill_char: ColChar) -> Vec<Point> {
    points
        .iter()
        .map(|point| Point::new(*point, fill_char))
        .collect()
}

/// Extract the positions from a vector of [`Point`]s
pub fn pixels_to_points(pixels: Vec<Point>) -> Vec<Vec2D> {
    pixels.iter().map(|p| p.pos).collect()
}

/// Draw a pseudo-line between the independent and dependent positions. Used by [`Triangle`](super::super::Triangle)
#[deprecated = "Please use `Triangle::interpolate` instead"]
pub fn interpolate(i0: isize, d0: f64, i1: isize, d1: f64) -> Vec<isize> {
    super::super::Triangle::interpolate(i0, d0, i1, d1)
}

/// Returns true if the [`Vec2D`]s in the vector are arranged clockwise
pub fn is_clockwise(points: &[Vec2D]) -> bool {
    let mut m = vec![];
    for i in 0..points.len() {
        let (p1, p2) = (points[i], points[(i + 1) % points.len()]);
        m.push((p1.x - p2.x) * (p1.y + p2.y));
    }

    m.iter().sum::<isize>() <= 0
}

/// Wrapping is used to determine how you want to handle out-of-bounds pixels during plotting pixels to the screen. Here's how each possible value functions:
#[derive(Debug, Clone, Copy)]
pub enum Wrapping {
    /// `Wrapping::Wrap` wraps any out of bounds pixels around to the other side. This is useful if you have an object that travels the entirety of the screen and appears on the other side when it reaches the end.
    Wrap,
    /// `Wrapping::Ignore` simply skips all out-of-bounds pixels. This is useful if you might have an object clipping through the edge of the screen but don't want it to appear on the other side like with [`Wrapping::Wrap`]
    Ignore,
    /// `Wrapping::Panic` will `panic!` if any pixels are out of bounds. You should use this if you have your own wrapping system implemented
    Panic,
}
