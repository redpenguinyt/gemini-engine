//! A `GeminiEngine` (py) example, recreated in its rusty successor
//! You can find the original here:
//! <https://github.com/redpenguinyt/GeminiExamples/blob/master/test_multiple_movement.py>

use std::{thread, time::Duration};

use gemini_engine::{
    elements::{
        view::{ColChar, View, Wrapping},
        Rect, Vec2D,
    },
    fps_gameloop,
};

const BLOCK_SIZE: Vec2D = Vec2D::new(4, 2);
const FILL_CHAR: ColChar = ColChar::SOLID;

fn main() {
    let mut view = View::new(50, 12, ColChar::BACKGROUND);

    let mut blocks = vec![
        Rect::new(Vec2D::new(0, 0), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 2), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 4), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 6), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 8), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 10), BLOCK_SIZE, FILL_CHAR),
    ];

    let mut i = 0;
    fps_gameloop!(
        {
            i += 1;
            for (j, block) in (0u32..).zip(blocks.iter_mut()) {
                if i % 2_u32.pow(j) == 0 {
                    block.pos.x += 1;
                }
            }
        },
        {
            view.clear();
            for block in &blocks {
                view.blit(block, Wrapping::Wrap);
            }
            let _ = view.display_render();

            if blocks
                .iter()
                .all(|b| b.pos.x % isize::try_from(view.width).expect("Wrapped usize") == 0)
            {
                thread::sleep(Duration::from_secs(2));
            };
        },
        200.0
    );
}
