use super::{ColChar, Vec2D, ViewElement};

/// The `Point` holds a single [`Vec2D`] (the coordinates at which it is printed when blit to a [`View`]) and a [ColChar]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub pos: Vec2D,
    pub fill_char: ColChar,
}

impl Point {
    pub fn new(pos: Vec2D, fill_char: ColChar) -> Self {
        Self { pos, fill_char }
    }
}

impl From<(Vec2D, ColChar)> for Point {
    fn from(value: (Vec2D, ColChar)) -> Self {
        Self {
            pos: value.0,
            fill_char: value.1,
        }
    }
}

impl ViewElement for Point {
    fn active_pixels(&self) -> Vec<Point> {
        vec![*self]
    }
}
