use super::{remove_leading_newlines, Sprite};
use crate::elements::{
    view::{Modifier, ViewElement},
    Pixel, Vec2D,
};

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
        let processed_frames: Vec<String> = frames
            .iter()
            .map(|frame| remove_leading_newlines(frame))
            .collect();

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
