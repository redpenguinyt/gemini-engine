//! `quick-start.rs` recreated using [`gameloop::MainLoopRoot`](gemini_engine::gameloop::MainLoopRoot)

use gemini_engine::elements::{
    view::{ColChar, View, Wrapping},
    Pixel, Vec2D,
};
use gemini_engine::gameloop::MainLoopRoot;

const FPS: f32 = 30.0;

struct Game {
    view: View,
    pixel: Pixel,
}

impl Game {
    fn new() -> Self {
        Self {
            view: View::new(40, 8, ColChar::BACKGROUND),
            pixel: Pixel::new(Vec2D { x: 10, y: 5 }, ColChar::SOLID),
        }
    }
}

impl MainLoopRoot for Game {
    type InputDataType = bool; // dummy type, since it isn't used in this project
    fn frame(&mut self, _input_data: Option<Self::InputDataType>) {
        self.pixel.pos.x += 1;
    }
    fn render_frame(&mut self) {
        self.view.clear();
        self.view.blit(&self.pixel, Wrapping::Wrap);
        let _ = self.view.display_render();
    }
}

fn main() {
    let mut game = Game::new();

    game.main_loop(FPS);
}
