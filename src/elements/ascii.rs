//! This module holds the structs related to display of ASCII characters, both text and ASCII art

use super::view::{ColChar, Modifier, Pixel, Vec2D, ViewElement};

/// Remove all leading newlines from the string
pub fn remove_leading_newlines(texture: &str) -> String {
    let mut texture: Vec<char> = texture.chars().rev().collect();

    while *texture.last().expect("Texture consists of only newlines") == '\n' {
        texture.pop();
    }

    texture.iter().rev().collect()
}

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
    pub fn with_align(pos: Vec2D, content: &str, align: TextAlign, modifier: Modifier) -> Text {
        let mut tmp = Text::new(pos, content, modifier);
        tmp.align = align;

        tmp
    }

    /// Return a vector of Pixels to display the given content
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
    pub fn draw_with_align(
        pos: Vec2D,
        content: &str,
        align: TextAlign,
        modifier: Modifier,
    ) -> Vec<Pixel> {
        let pos = match align {
            TextAlign::Left => pos,
            TextAlign::Centered => pos - Vec2D::new(content.len() as isize / 2, 0),
            TextAlign::Right => pos - Vec2D::new(content.len() as isize, 0),
        };

        Text::draw(pos, content, modifier)
    }
}

impl ViewElement for Text<'_> {
    fn active_pixels(&self) -> Vec<Pixel> {
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

/// The `AnimatedSprite` struct contains a list of `String`s into which it indexes based on its [`current_frame`](AnimatedSprite::current_frame) property. You can cycle through frames with the [`AnimatedSprite::next_frame()`](AnimatedSprite::next_frame()) function
pub struct AnimatedSprite {
    pub pos: Vec2D,
    /// A collection of frames - ACII textures to be displayed by the `AnimatedSprite`
    pub frames: Vec<String>,
    /// The current frame being displayed. This will index directly into [`frames`](AnimatedSprite::frames)
    pub current_frame: usize,
    /// A raw [`Modifier`], determining the appearance of the `AnimatedSprite`
    pub modifier: Modifier,
    // TODO: add x and y align
}

impl AnimatedSprite {
    /// Create a new `AnimatedSprite` struct. All newlines at the beginning of each texture will be removed
    pub fn new(pos: Vec2D, frames: &[&str], modifier: Modifier) -> Self {
        let processed_frames: Vec<String> =
            frames.iter().map(|frame| remove_leading_newlines(frame)).collect();

        Self {
            pos,
            frames: processed_frames,
            current_frame: 0,
            modifier,
        }
    }

    /// Go to the next frame of the `AnimatedSprite`'s frames. Will automatically wrap around at the end of the list
    pub fn next_frame(&mut self) {
        self.current_frame += 1;
        self.current_frame %= self.frames.len();
    }

    /// Returns true if the `current_frame` property is within range of the list of frames. Also returns false if the list of frames is empty
    pub fn is_within_frame_range(&self) -> bool {
        self.current_frame < self.frames.len()
    }
}

impl ViewElement for AnimatedSprite {
    fn active_pixels(&self) -> Vec<Pixel> {
        if !self.is_within_frame_range() {
            panic!(
                "AnimatedSprite tried indexing at {} in list of frames size {}",
                self.current_frame,
                self.frames.len()
            );
        }

        Sprite::draw(self.pos, &self.frames[self.current_frame], self.modifier)
    }
}
