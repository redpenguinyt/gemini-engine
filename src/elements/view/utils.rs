//! A module containing various helper functions and structs

use super::{ColChar, Pixel, Vec2D};

/// Combine a vector of [`Vec2D`]s and a single `fill_char` into a vector of `(Vec2D, char)` tuples, ready to return for `ViewElement::active_pixels`. Useful if your [`ViewElement`](super::ViewElement) only has one fill character across all of it
pub fn points_to_pixels(points: Vec<Vec2D>, fill_char: ColChar) -> Vec<Pixel> {
    points
        .iter()
        .map(|point| Pixel::new(*point, fill_char))
        .collect()
}

/// Extract the positions from a vector of [`Pixel`]s
pub fn pixels_to_points(pixels: Vec<Pixel>) -> Vec<Vec2D> {
    pixels.iter().map(|p| p.pos).collect()
}

/// Draw a pseudo-line between the independent and dependent positions.
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
