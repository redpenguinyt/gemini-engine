use std::{thread::sleep, time::Duration};

use gemini_engine::elements::{
    ascii::{TextAlign, TextAlign2D},
    view::{ColChar, Modifier, Wrapping, ScaleFitView},
    Text, Vec2D, Sprite,
};

const TEXTURE: &str = "
.-----.
|     |
| hi! |
|     |
`-----'";

fn main() {
    let mut scale_view = ScaleFitView::new(ColChar::BACKGROUND);

    let mut text = Text::new(Vec2D::ZERO, "This is some centered text!", Modifier::None)
        .with_align(TextAlign::Centered);

    let mut sprite = Sprite::new(Vec2D::ZERO, TEXTURE, Modifier::None).with_align(TextAlign2D::CENTERED);

    loop {
        text.pos = scale_view.intended_size()/2;
        sprite.pos = scale_view.intended_size()/2;
        sprite.pos.y -= 5;

        scale_view.update();
        scale_view.view.blit(&text, Wrapping::Wrap);
        scale_view.view.blit(&sprite, Wrapping::Wrap);
        scale_view.view.display_render().unwrap();

        sleep(Duration::from_millis(10))
    }
}
