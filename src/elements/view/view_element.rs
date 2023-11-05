use super::{utils, Pixel, Vec2D};

/// `ViewElement` is a trait that must be implemented by any element that can be blitted to a [`View`](super::View)
pub trait ViewElement {
    /// Return a vector of the element's [`Pixel`]s - A [`ColChar`](super::ColChar). If your whole object is a solid colour, consider using [`utils::points_to_pixels()`](super::utils::points_to_pixels()) which will add the same [`ColChar`](super::ColChar) to every point and can then be used as this function's output
    fn active_pixels(&self) -> Vec<Pixel>;

    /// Return the positions the `ViewElement` occupies, essentially [`active_pixels()`](ViewElement::active_pixels()) without the [`ColChar`](super::ColChar)s. This has a default setting that extracts the [`Vec2D`]s from [`active_pixels`](ViewElement::active_pixels()) but you can set it to something else to make it faster
    fn active_points(&self) -> Vec<Vec2D> {
        utils::pixels_to_points(self.active_pixels())
    }
}
