use crate::elements::{
    view::{ColChar, Modifier, ViewElement},
    Pixel, Vec2D,
};

use super::TextAlign;

/// Displays text at the given position
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Text {
    /// The position of the text. You can use [`Text::align`] to determine how it aligns to this position
    pub pos: Vec2D,
    /// The actual text content of the element
    pub content: String,
    /// How the content should align to the position
    pub align: TextAlign,
    /// A raw [`Modifier`], determining the appearance of the `Text`
    pub modifier: Modifier,
}

impl Text {
    /// Create a new Text element with a position, content and modifier
    ///
    /// # Panics
    /// This function will panic if the content contains a newline, as Text only works with single lines. For multi-line strings, see [Sprite](super::Sprite)
    #[must_use]
    pub fn new(pos: Vec2D, content: &str, modifier: Modifier) -> Self {
        assert!(
            !content.contains('\n'),
            "Text was created with a content string containing a \n character"
        );

        Self {
            pos,
            content: String::from(content),
            align: TextAlign::Begin,
            modifier,
        }
    }

    /// Return the `Text` with the modified align property
    #[must_use]
    pub const fn with_align(self, align: TextAlign) -> Self {
        let mut tmp = self;
        tmp.align = align;
        tmp
    }

    /// Return a vector of Pixels to display the given content
    #[must_use]
    pub fn draw(pos: Vec2D, content: &str, modifier: Modifier) -> Vec<Pixel> {
        let mut pixels = vec![];
        for (x, text_char) in content.chars().enumerate() {
            if text_char != ' ' {
                pixels.push(Pixel::new(
                    pos + Vec2D::new(x as isize, 0),
                    ColChar {
                        text_char,
                        modifier,
                    },
                ));
            }
        }

        pixels
    }

    /// Return a vector of Pixels to display the given content, aligning the content to the position as directed by the `align` attribute
    #[must_use]
    pub fn draw_with_align(
        pos: Vec2D,
        content: &str,
        align: TextAlign,
        modifier: Modifier,
    ) -> Vec<Pixel> {
        let pos = Vec2D::new(align.apply_to(pos.x, content.len() as isize), pos.y);

        Self::draw(pos, content, modifier)
    }
}

impl ViewElement for Text {
    fn active_pixels(&self) -> Vec<Pixel> {
        Self::draw_with_align(self.pos, &self.content, self.align, self.modifier)
    }
}
