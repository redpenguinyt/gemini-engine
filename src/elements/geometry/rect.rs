use crate::elements::view::{utils, ColChar, Pixel, Vec2D, ViewElement};

/// The `Rect` takes a position and size, and returns a rectangle at that position with the given width and size when blit to a [`View`](super::super::View)
pub struct Rect {
    /// The position of the top-left corner of the `Rect`
    pub pos: Vec2D,
    /// The size of the `Rect`, extending from [`Rect::pos`]
    pub size: Vec2D,
    pub fill_char: ColChar,
}

impl Rect {
    pub const fn new(pos: Vec2D, size: Vec2D, fill_char: ColChar) -> Self {
        Self {
            pos,
            size,
            fill_char,
        }
    }

    /// Draw a Rectangle with a given position (representing the top-left corner) and size
    pub fn draw(pos: Vec2D, size: Vec2D) -> Vec<Vec2D> {
        (0..size.x)
            .flat_map(|x| (0..size.y).map(move |y| pos + Vec2D { x, y }))
            .collect()
    }
}

impl ViewElement for Rect {
    fn active_pixels(&self) -> Vec<Pixel> {
        utils::points_to_pixels(self.active_points(), self.fill_char)
    }

    fn active_points(&self) -> Vec<Vec2D> {
        Rect::draw(self.pos, self.size)
    }
}
