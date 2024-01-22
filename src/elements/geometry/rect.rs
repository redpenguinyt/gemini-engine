use crate::elements::view::{utils, ColChar, Pixel, Vec2D, ViewElement};

/// The `Rect` takes a position and size, and returns a rectangle at that position with the given width and size when blit to a [`View`](super::super::View)
pub struct Rect {
    /// The position of the top-left corner of the `Rect`
    pub pos: Vec2D,
    /// The size of the `Rect`, extending from [`Rect::pos`]
    pub size: Vec2D,
    /// The [`ColChar`] used to fill the rectangle
    pub fill_char: ColChar,
}

impl Rect {
    /// Create a new rectangle using a given position, size and [`ColChar`]
    #[must_use]
    pub const fn new(pos: Vec2D, size: Vec2D, fill_char: ColChar) -> Self {
        Self {
            pos,
            size,
            fill_char,
        }
    }

    /// Create a new rectangle between two position to fill with a [`ColChar`]
    #[must_use]
    pub fn new_from_to(pos0: Vec2D, pos1: Vec2D, fill_char: ColChar) -> Self {
        Self::new(pos0, pos1 - pos0, fill_char)
    }

    /// Draw a Rectangle with a given position (representing the top-left corner) and size
    #[must_use]
    pub fn draw(pos: Vec2D, size: Vec2D) -> Vec<Vec2D> {
        (0..size.x)
            .flat_map(|x| (0..size.y).map(move |y| pos + Vec2D { x, y }))
            .collect()
    }
}

impl ViewElement for Rect {
    fn active_pixels(&self) -> Vec<Pixel> {
        utils::points_to_pixels(&self.active_points(), self.fill_char)
    }

    fn active_points(&self) -> Vec<Vec2D> {
        Self::draw(self.pos, self.size)
    }
}
