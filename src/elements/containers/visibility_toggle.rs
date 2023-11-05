use crate::elements::{view::ViewElement, Pixel, Vec2D};

/// `VisibilityToggle` is a container for a [`ViewElement`] with a property `visible`. When blit to the view the contained element will only appear if `visible` is `true`
#[derive(Debug, Clone)]
pub struct VisibilityToggle<E: ViewElement> {
    /// The element held by the `VisibilityToggle`. Must implement [`ViewElement`]
    pub element: E,
    /// Whether the element is visible
    pub visible: bool,
}

impl<E: ViewElement> VisibilityToggle<E> {
    /// Creates a new `VisibilityToggle` with the visibility set to true
    pub const fn new(element: E) -> Self {
        Self {
            element,
            visible: true,
        }
    }
}

impl<T: ViewElement> ViewElement for VisibilityToggle<T> {
    fn active_pixels(&self) -> Vec<Pixel> {
        match self.visible {
            true => self.element.active_pixels(),
            false => vec![],
        }
    }

    fn active_points(&self) -> Vec<Vec2D> {
        match self.visible {
            true => self.element.active_points(),
            false => vec![],
        }
    }
}
