use super::Pixel;

/// `ViewElement` is a trait that must be implemented by any element that can be blitted to a [`View`](super::View)
pub trait ViewElement {
    /// Return a vector of the element's [`Pixel`]s - A [`ColChar`](super::ColChar). If your whole object is a solid colour, consider using [`utils::points_to_pixels()`](super::utils::points_to_pixels()) which will add the same [`ColChar`](super::ColChar) to every point and can then be used as this function's output
    fn active_pixels(&self) -> Vec<Pixel>;
}
