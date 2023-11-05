use std::{thread::sleep, time::Duration};

use gemini_engine::elements::{
    ascii::TextAlign,
    view::{ColChar, Modifier, Wrapping},
    Text, View,
};

fn main() {
    let mut view = View::new(100, 100, ColChar::BACKGROUND).with_block_until_resized(true);
    let mut text = Text::with_align(
        view.center(),
        "This is some centered text!",
        TextAlign::Centered,
        Modifier::None,
    );

    loop {
        let terminal_size = termsize::get().unwrap();
        view.width = terminal_size.cols as usize;
        view.height = terminal_size.rows as usize - 2;

        text.pos = view.center();

        view.clear();
        view.blit(&text, Wrapping::Wrap);
        view.display_render().unwrap();

        sleep(Duration::from_millis(10))
    }
}
