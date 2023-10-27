//! This module holds the structs related to display of ASCII characters, both text and ASCII art

mod animated_sprite;
pub use animated_sprite::AnimatedSprite;

mod sprite;
pub use sprite::Sprite;

mod text;
pub use text::Text;

mod text_align;
pub use text_align::TextAlign;

/// Remove all leading newlines from the string
pub fn remove_leading_newlines(texture: &str) -> String {
    let mut texture: Vec<char> = texture.chars().rev().collect();

    while *texture.last().expect("Texture consists of only newlines") == '\n' {
        texture.pop();
    }

    texture.iter().rev().collect()
}
