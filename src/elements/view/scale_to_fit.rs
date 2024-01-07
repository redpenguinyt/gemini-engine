use super::{ColChar, Vec2D, View};
use crate::utils;

/// A wrapper around a [`View`] which auto resizes to fit the terminal window
///
/// The wrapper's [`update()`](ScaleFitView::update()) function replaces the `View`'s `clear()` function to handle the resizing
pub struct ScaleFitView {
    /// The [`View`] that this struct wraps around
    pub view: View,
    /// How many rows to leave clear below the rendered view. You might want to set this if you have more than one line of text after rendered text
    pub empty_row_count: isize,
}

impl Default for ScaleFitView {
    fn default() -> Self {
        ScaleFitView {
            view: View::new(0, 0, ColChar::EMPTY),
            empty_row_count: 1,
        }
    }
}

impl ScaleFitView {
    /// Create a new `ScaleFitView` with the given background `ColChar`
    pub fn new(background_char: ColChar) -> ScaleFitView {
        ScaleFitView {
            view: View::new(0, 0, background_char),
            ..Default::default()
        }
    }

    /// Returns the `ScaleFitView` with the updated [`empty_row_count`](ScaleFitView::empty_row_count)
    pub fn with_empty_row_count(mut self, empty_row_count: isize) -> ScaleFitView {
        self.empty_row_count = empty_row_count;
        self
    }

    /// Returns the size of the terminal, with the y adjusted as intended using the [`empty_row_count`](ScaleFitView::empty_row_count) property
    pub fn intended_size(&self) -> Vec2D {
        let mut term_size = utils::get_termsize_as_vec2d().expect("Failed to get terminal size");
        term_size.y -= self.empty_row_count + 1;
        term_size
    }

    /// Resize and clear the `View`
    pub fn update(&mut self) {
        let term_size = self.intended_size();
        self.view.width = term_size.x as usize;
        self.view.height = term_size.y as usize;

        self.view.clear()
    }
}
