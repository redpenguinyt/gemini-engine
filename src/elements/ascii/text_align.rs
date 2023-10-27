use crate::elements::Vec2D;

/// An enum to set the alignment of a Text element's content
#[derive(Debug, Clone, Copy)]
pub enum TextAlign {
    /// Align to the beginning of the text
    Left,
    /// Align to the center of the text
    Centered,
    /// Align to the end of the text
    Right,
}

impl TextAlign {
    /// Align the given position as dictated by the `TextAlign` enum variation
    pub fn apply_to(&self, pos: Vec2D, text_length: isize) -> Vec2D {
        match self {
            TextAlign::Left => pos,
            TextAlign::Centered => pos - Vec2D::new(text_length / 2, 0),
            TextAlign::Right => pos - Vec2D::new(text_length, 0),
        }
    }
}
