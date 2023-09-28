//! This module holds the structs related to display of ASCII characters, both text and ASCII art

use super::view::{ColChar, Modifier, Point, Vec2D, ViewElement};

#[derive(Debug, Clone, Copy)]
pub enum TextAlign {
    Left,
    Centered,
    Right,
}

/// Displays text at the given position
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub pos: Vec2D,
    pub content: &'a str,
    /// A raw [`Modifier`], determining the appearance of the `Sprite`
    pub align: TextAlign,
    pub modifier: Modifier,
}

impl<'a> Text<'a> {
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
}
impl Sprite {
    pub fn new(pos: Vec2D, texture: &str, modifier: Modifier) -> Self {
        let mut texture = String::from(texture);
        if texture.starts_with('\n') {
            texture.pop();
        }
        Self {
            pos,
            texture,
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
