//! A module containing various helper functions and structs
use super::{ColChar, Pixel, Vec2D};
pub use crate::utils::get_terminal_size_as_vec2d;

/// Combine a vector of [`Vec2D`]s and a single `fill_char` into a vector of `(Vec2D, char)` tuples, ready to return for `ViewElement::active_pixels`. Useful if your [`ViewElement`](super::ViewElement) only has one fill character across all of it
#[must_use]
pub fn points_to_pixels(points: &[Vec2D], fill_char: ColChar) -> Vec<Pixel> {
    points
        .iter()
        .map(|point| Pixel::new(*point, fill_char))
        .collect()
}

/// Extract the positions from a vector of [`Pixel`]s
#[must_use]
pub fn pixels_to_points(pixels: &[Pixel]) -> Vec<Vec2D> {
    pixels.iter().map(|p| p.pos).collect()
}

/// Draw a pseudo-line between the independent and dependent positions.
#[deprecated = "Please use `Triangle::interpolate` instead"]
#[must_use]
pub fn interpolate(i0: isize, d0: f64, i1: isize, d1: f64) -> Vec<isize> {
    super::super::Triangle::interpolate(i0, d0 as isize, i1, d1 as isize)
}

/// Returns true if the [`Vec2D`]s in the vector are arranged clockwise
#[must_use]
pub fn is_clockwise(points: &[Vec2D]) -> bool {
    let mut m = vec![];
    for i in 0..points.len() {
        let (p1, p2) = (points[i], points[(i + 1) % points.len()]);
        m.push((p1.x - p2.x) * (p1.y + p2.y));
    }

    m.iter().sum::<isize>() <= 0
}
