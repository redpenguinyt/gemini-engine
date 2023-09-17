use super::view::{ColChar, Modifier, Point, Vec2D, ViewElement};

/// Displays text starting at the given position
#[non_exhaustive]
pub struct Text<'a> {
    pub pos: Vec2D,
    pub content: &'a str,
    pub modifier: Modifier,
}

impl Text<'_> {
    pub fn new(pos: Vec2D, content: &str, modifier: Modifier) -> Text {
        assert!(!content.contains('\n'));

        Text {
            pos,
            content,
            modifier,
        }
    }
}

impl ViewElement for Text<'_> {
    fn active_pixels(&self) -> Vec<Point> {
        let mut pixels = vec![];
        for (x, char) in self.content.chars().enumerate() {
            if char != ' ' {
                pixels.push(Point::new(
                    self.pos + Vec2D::new(x as isize, 0),
                    ColChar {
                        fill_char: char,
                        modifier: self.modifier,
                    },
                ));
            }
        }

        pixels
    }
}

/// A `ViewElement` that takes a multi-line string as a parameter, and can be used to put ASCII art, text and other such things on the View
#[non_exhaustive]
pub struct Sprite {
    pub pos: Vec2D,
    pub texture: String,
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
            pixels.extend(
                Text::new(self.pos + Vec2D::new(0, y as isize), line, self.modifier)
                    .active_pixels(),
            );
        }

        pixels
    }
}
