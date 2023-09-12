use super::Colour;
use std::fmt::Display;

/// The `Modifier` enum is used for adding modifications to text such as colour, bold/italic/underline and others. It's essentially a wrapper for `\x1b[{x}m`, where {x} is a code or rgb value of some sort. `Modifier` is primarily used by [`ColChar`](super::ColChar) as one of its properties
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    Coded(u8),
    Colour(Colour),
    None,
}

impl Modifier {
    pub const END: Self = Self::Coded(0);
    pub const BOLD: Self = Self::Coded(1);
    pub const LIGHT: Self = Self::Coded(2);
    pub const ITALIC: Self = Self::Coded(3);
    pub const UNDERLINE: Self = Self::Coded(4);
    pub const INVERTED: Self = Self::Coded(7);
    pub const CROSSED: Self = Self::Coded(9);
    pub const RED: Self = Self::Coded(31);
    pub const GREEN: Self = Self::Coded(32);
    pub const YELLOW: Self = Self::Coded(33);
    pub const BLUE: Self = Self::Coded(34);
    pub const PURPLE: Self = Self::Coded(35);
    pub const CYAN: Self = Self::Coded(36);

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Colour(Colour::rgb(r, g, b))
    }

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
