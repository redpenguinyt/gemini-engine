use super::Vec2D;

/// Wrapping is used to determine how you want to handle out-of-bounds pixels during plotting pixels to the screen. Here's how each possible value functions:
#[derive(Debug, Clone, Copy)]
pub enum Wrapping {
    /// `Wrapping::Wrap` wraps any out of bounds pixels around to the other side. This is useful if you have an object that travels the entirety of the screen and appears on the other side when it reaches the end.
    Wrap,
    /// `Wrapping::Ignore` simply skips all out-of-bounds pixels. This is useful if you might have an object clipping through the edge of the screen but don't want it to wrap to the other side like [`Wrapping::Wrap`] or panic and end the process like [`Wrapping::Panic`]
    Ignore,
    /// `Wrapping::Panic` will `panic!` if any pixels are out of bounds. You should use this if you have your own wrapping system implemented
    Panic,
}

impl Wrapping {
    /// Handle the position based on the given bounds and the Wrapping variation (See the [Wrapping] documentation for more info)
    pub fn handle_bounds(&self, pos: Vec2D, bounds: Vec2D) -> Option<Vec2D> {
        let in_bounds_pos = pos % bounds;

        match self {
            Wrapping::Wrap => Some(in_bounds_pos),
            Wrapping::Ignore => {
                if pos != in_bounds_pos {
                    None
                } else {
                    Some(pos)
                }
            }
            Wrapping::Panic => {
                if pos != in_bounds_pos {
                    panic!("{} is out of bounds", pos);
                } else {
                    Some(pos)
                }
            }
        }
    }
}
