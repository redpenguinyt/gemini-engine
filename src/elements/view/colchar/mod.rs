use std::fmt::{Debug, Display};
mod modifier;
pub use modifier::Modifier;
mod colour;
pub use colour::Colour;

/// We use `ColChar` to say exactly what each pixel should look like and what colour it should be. That is, the [`View`](super::View)'s canvas is just a vector of `ColChar`s under the hood. `ColChar` has the [`fill_char`](ColChar::fill_char) and [`modifier`](ColChar::modifier) properties. [`fill_char`](ColChar::fill_char) is the single ascii character used as the "pixel" when the [`View`](super::View) is rendered, whereas [`modifier`](ColChar::modifier) can give that pixel a colour or make it bold/italic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColChar {
    pub fill_char: char,
    pub modifier: Modifier,
}

impl ColChar {
    pub const SOLID: Self = Self {
        fill_char: '█',
        modifier: Modifier::None,
    };
    pub const BACKGROUND: Self = Self {
        fill_char: '░',
        modifier: Modifier::None,
    };
    pub const EMPTY: Self = Self {
        fill_char: ' ',
        modifier: Modifier::None,
    };
    /// For use with the [`Sprite`](crate::elements::Sprite) and [`Text`](crate::elements::Text) elements, which consider a regular whitespace a transparent character
    pub const VOID: Self = Self {
        fill_char: '\u{2008}',
        modifier: Modifier::None,
    };

    pub fn new(fill_char: char, modifier: Modifier) -> Self {
        Self {
            fill_char,
            modifier,
        }
    }

    // Return the rendered ColChar
    #[deprecated = "Please use `ColChar`'s implementation of `std::fmt::Display` instead"]
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Return a ColChar with the same `modifier` and new `fill_char`
    pub fn with_char(&self, fill_char: char) -> Self {
        Self {
            fill_char,
            modifier: self.modifier,
        }
    }

    /// Return a ColChar with the same `fill_char` and new `modifier`
    pub fn with_mod(&self, modifier: Modifier) -> Self {
        Self {
            fill_char: self.fill_char,
            modifier,
        }
    }

    /// Return a ColChar with the same `fill_char` and new `modifier` of the `Modifier::Colour` enum variant from RGB values
    pub fn with_rgb(&self, r: u8, g: u8, b: u8) -> Self {
        Self {
            fill_char: self.fill_char,
            modifier: Modifier::from_rgb(r, g, b),
        }
    }

    /// Return a ColChar with the same `fill_char` and new `modifier` of the `Modifier::Colour` enum variant from HSV values
    pub fn with_hsv(&self, h: u8, s: u8, v: u8) -> Self {
        Self {
            fill_char: self.fill_char,
            modifier: Modifier::from_hsv(h, s, v),
        }
    }

    /// Return a ColChar with the same `fill_char` and new `modifier` of the `Modifier::Colour` enum variant from an HSV value
    pub fn with_colour(&self, colour: Colour) -> Self {
        Self {
            fill_char: self.fill_char,
            modifier: Modifier::Colour(colour),
        }
    }
}

impl Display for ColChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.modifier {
            Modifier::None => write!(f, "{}", self.fill_char),
            _ => write!(f, "{}{}{}", self.modifier, self.fill_char, Modifier::END),
        }
    }
}
