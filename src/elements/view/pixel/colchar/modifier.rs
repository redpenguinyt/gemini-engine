use super::Colour;
use std::fmt::Display;

/// The `Modifier` enum is used for adding modifications to text such as colour, bold/italic/underline and others. It's essentially a wrapper for `\x1b[{x}m`, where {x} is a code or rgb value of some sort. `Modifier` is primarily used by [`ColChar`](super::ColChar) as one of its properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Modifier {
    /// Represents a [`Modifier`] by an escape code. A `Modifier::Coded(31)` would return a `\x1b[31m`.
    ///
    /// See <https://prirai.github.io/blogs/ansi-esc/#colors-graphics-mode> for codes you can use
    Coded(u8),
    /// Represents a `Modifier` by a [`Colour`], which itself is an RGB value
    Colour(Colour),
    /// Represents a lack of `Modifier`, if you don't want the pixel to be coloured or decorated in any way
    #[default]
    None,
}

impl Modifier {
    /// An END code, this clears all previously applied modifiers. You should never have to use this yourself as `View` makes use of it between pixels where necessary
    pub const END: Self = Self::Coded(0);
    /// A `Modifier` with a red ANSI escape code
    pub const RED: Self = Self::Coded(31);
    /// A Modifier with a green ANSI escape code
    pub const GREEN: Self = Self::Coded(32);
    /// A Modifier with a yellow ANSI escape code
    pub const YELLOW: Self = Self::Coded(33);
    /// A Modifier with a blue ANSI escape code
    pub const BLUE: Self = Self::Coded(34);
    /// A Modifier with a purple ANSI escape code
    pub const PURPLE: Self = Self::Coded(35);
    /// A Modifier with a cyan ANSI escape code
    pub const CYAN: Self = Self::Coded(36);

    /// Create a `Modifier::Colour` from an RGB value
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Colour(Colour::rgb(r, g, b))
    }

    /// Create a `Modifier::Colour` from an HSV value
    pub fn from_hsv(h: u8, s: u8, v: u8) -> Self {
        Self::Colour(Colour::hsv(h, s, v))
    }
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coded(code) => write!(f, "\x1b[{}m", code),
            Self::Colour(c) => write!(f, "\x1b[38;2;{};{};{}m", c.r, c.g, c.b),
            Self::None => Ok(()),
        }
    }
}
