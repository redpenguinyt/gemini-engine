use super::{ColChar, Vec2D, ViewElement};

#[deprecated = "Renamed to Pixel, please use that instead"]
pub type Point = Pixel;

/// The `Pixel` holds a single [`Vec2D`] (the coordinates at which it is printed when blit to a [`View`](super::View)) and a [`ColChar`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    /// The position of the `Pixel`
    pub pos: Vec2D,
    /// The appearance/colour of the `Pixel`
    pub fill_char: ColChar,
}

impl Pixel {
    /// Create a new `Pixel` from a [`Vec2D`] and [`ColChar`]
    pub const fn new(pos: Vec2D, fill_char: ColChar) -> Self {
        Self { pos, fill_char }
    }
}

impl From<(Vec2D, ColChar)> for Pixel {
    fn from(value: (Vec2D, ColChar)) -> Self {
        Self {
            pos: value.0,
            fill_char: value.1,
        }
    }
}

impl ViewElement for Pixel {
    fn active_pixels(&self) -> Vec<Pixel> {
        vec![*self]
    }
}
