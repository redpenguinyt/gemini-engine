use super::{remove_leading_newlines, Text, TextAlign2D};
use crate::elements::{
    view::{Modifier, ViewElement},
    Pixel, Vec2D,
};

/// The `Sprite` takes a multi-line string as a parameter, and can be used to put ASCII art, text and other such things on the `View`
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Sprite {
    /// The position from which the sprite will be drawn from
    pub pos: Vec2D,
    /// The ACII texture (pun intended) displayed by the `Sprite`
    pub texture: String,
    /// A raw [`Modifier`], determining the appearance of the `Sprite`
    pub modifier: Modifier,
    /// How the Sprite should align to the position
    pub align: TextAlign2D,
}

impl Sprite {
    /// Create a new `Sprite` struct. All newlines at the beginning of the texture will be removed
    #[must_use]
    pub fn new(pos: Vec2D, texture: &str, modifier: Modifier) -> Self {
        Self {
            pos,
            texture: remove_leading_newlines(texture),
            modifier,
            align: TextAlign2D::default(),
        }
    }

    /// Return the `Sprite` with the modified align property
    #[must_use]
    pub const fn with_align(self, align: TextAlign2D) -> Self {
        let mut tmp = self;
        tmp.align = align;
        tmp
    }

    /// Render a string texture at a given position in a [`ViewElement::active_pixels()`]-readable format
    #[must_use]
    pub fn draw(pos: Vec2D, texture: &str, modifier: Modifier) -> Vec<Pixel> {
        let mut pixels = vec![];

        let lines = texture.split('\n');
        for (y, line) in (0isize..).zip(lines) {
            pixels.extend(Text::draw(pos + Vec2D::new(0, y), line, modifier));
        }

        pixels
    }

    /// Return a vector of Pixels to display the given content, aligning the content to the position as directed by the `align` attribute
    #[must_use]
    pub fn draw_with_align(
        pos: Vec2D,
        texture: &str,
        align: TextAlign2D,
        modifier: Modifier,
    ) -> Vec<Pixel> {
        let content_size = Vec2D::new(
            texture.lines().count() as isize,
            texture.lines().map(|line| line.len()).max().unwrap_or(0) as isize,
        );
        let pos = align.apply_to(pos, content_size);

        Self::draw(pos, texture, modifier)
    }
}

impl ViewElement for Sprite {
    fn active_pixels(&self) -> Vec<Pixel> {
        Self::draw_with_align(self.pos, &self.texture, self.align, self.modifier)
    }
}
