use super::{remove_leading_newlines, Text};
use crate::elements::{
    view::{Modifier, ViewElement},
    Pixel, Vec2D,
};

/// The `Sprite` takes a multi-line string as a parameter, and can be used to put ASCII art, text and other such things on the `View`
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Sprite {
    pub pos: Vec2D,
    /// The ACII texture (pun intended) displayed by the `Sprite`
    pub texture: String,
    /// A raw [`Modifier`], determining the appearance of the `Sprite`
    pub modifier: Modifier,
    // TODO: add x and y align
}
impl Sprite {
    /// Create a new `Sprite` struct. All newlines at the beginning of the texture will be removed
    pub fn new(pos: Vec2D, texture: &str, modifier: Modifier) -> Self {
        Self {
            pos,
            texture: remove_leading_newlines(texture),
            modifier,
        }
    }

    pub fn draw(pos: Vec2D, texture: &str, modifier: Modifier) -> Vec<Pixel> {
        let mut pixels = vec![];

        let lines = texture.split('\n');
        for (y, line) in lines.enumerate() {
            pixels.extend(Text::draw(pos + Vec2D::new(0, y as isize), line, modifier));
        }

        pixels
    }
}

impl ViewElement for Sprite {
    fn active_pixels(&self) -> Vec<Pixel> {
        Self::draw(self.pos, &self.texture, self.modifier)
    }
}
