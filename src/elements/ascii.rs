//! This module holds the structs related to display of ASCII characters, both text and ASCII art

use super::view::{ColChar, Modifier, Point, Vec2D, ViewElement};

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

/// Displays text at the given position
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub pos: Vec2D,
    /// The actual text content of the element
    pub content: &'a str,
    /// How the content should align to the position
    pub align: TextAlign,
    /// A raw [`Modifier`], determining the appearance of the `Sprite`
    pub modifier: Modifier,
}

impl<'a> Text<'a> {
    /// Create a new Text element with a position, content and modifier
    ///
    /// # Panics
    /// This function will panic if the content contains a newline, as Text only works with single lines. For multi-line strings, see [Sprite]
    pub fn new(pos: Vec2D, content: &str, modifier: Modifier) -> Text {
        if content.contains('\n') {
            panic!("Text was created with a content string containing a \n character")
        }

        Text {
            pos,
            content,
            align: TextAlign::Left,
            modifier,
        }
    }

    /// Create a `Text` element with an [`align`](Text::align) parameter to set the `Text`'s align (see the [TextAlign] documentation)
    ///
    /// # Panics
    /// This function will panic if the content contains a newline, as Text only works with single lines. For multi-line strings, see [Sprite]
    pub fn new_with_align(pos: Vec2D, content: &str, align: TextAlign, modifier: Modifier) -> Text {
        let mut tmp = Text::new(pos, content, modifier);
        tmp.align = align;

        tmp
    }

    /// Return a vector of Points to display the given content
    pub fn draw(pos: Vec2D, content: &str, modifier: Modifier) -> Vec<Point> {
        let mut pixels = vec![];
        for (x, text_char) in content.chars().enumerate() {
            if text_char != ' ' {
                pixels.push(Point::new(
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

    /// Return a vector of Points to display the given content, aligning the content to the position as directed by the `align` attribute
    pub fn draw_with_align(
        pos: Vec2D,
        content: &str,
        align: TextAlign,
        modifier: Modifier,
    ) -> Vec<Point> {
        let pos = match align {
            TextAlign::Left => pos,
            TextAlign::Centered => pos - Vec2D::new(content.len() as isize / 2, 0),
            TextAlign::Right => pos - Vec2D::new(content.len() as isize, 0),
        };

        Text::draw(pos, content, modifier)
    }
}

impl ViewElement for Text<'_> {
    fn active_pixels(&self) -> Vec<Point> {
        Text::draw(self.pos, self.content, self.modifier)
    }
}

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
        let mut texture: Vec<char> = texture.chars().rev().collect();

        while *texture.last().expect("Texture consists of only newlines") == '\n' {
            texture.pop();
        }

        Self {
            pos,
            texture: texture.iter().rev().collect(),
            modifier,
        }
    }
}

impl ViewElement for Sprite {
    fn active_pixels(&self) -> Vec<Point> {
        let mut pixels = vec![];

        let lines = self.texture.split('\n');
        for (y, line) in lines.enumerate() {
            pixels.extend(Text::draw(
                self.pos + Vec2D::new(0, y as isize),
                line,
                self.modifier,
            ));
        }

        pixels
    }
}
