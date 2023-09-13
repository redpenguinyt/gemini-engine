//! A GeminiEngine (py) example, recreated in its rusty successor
//! You can find the original here:
//! https://github.com/redpenguinyt/GeminiExamples/blob/master/test_multiple_movement.py
use std::{thread, time::Duration};

use gemini_engine::{elements::{Rect, Vec2D, view::{View, ColChar, Wrapping}}, fps_gameloop};

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
	fps_gameloop!({
		i += 1;
		blocks[0].pos.x += 1;
		if i % 2 == 0 { blocks[1].pos.x += 1; }
		if i % 4 == 0 { blocks[2].pos.x += 1; }
		if i % 8 == 0 { blocks[3].pos.x += 1; }
		if i % 16 == 0 { blocks[4].pos.x += 1; }
		if i % 32 == 0 { blocks[5].pos.x += 1; }
	}, {
		view.clear();
		for block in &blocks {
			view.blit(block, Wrapping::Wrap);
		}
		view.display_render().unwrap();

		if blocks.iter().all(|b| b.pos.x % view.width as isize == 0) {
			thread::sleep(Duration::from_secs(2));
		};
	}, 200);
}