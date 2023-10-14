//! `quick-start.rs` recreated using [`gameloop::MainLoopRoot`](gemini_engine::gameloop::MainLoopRoot)

use gemini_engine::elements::{
    view::{ColChar, View, Wrapping},
    Pixel, Vec2D,
};
use gemini_engine::gameloop::MainLoopRoot;

const FPS: f32 = 30.0;

struct Game {
    view: View,
    point: Pixel,
}

impl Game {
    fn new() -> Game {
        Game {
            view: View::new(40, 8, ColChar::BACKGROUND),
            point: Pixel::new(Vec2D { x: 10, y: 5 }, ColChar::SOLID),
        }
    }
}

impl MainLoopRoot for Game {
    type InputDataType = bool; // dummy type, since it isn't used in this project
    fn frame(&mut self, _input_data: Option<Self::InputDataType>) {
        self.point.pos.x += 1;
    }
    fn render_frame(&mut self) {
        self.view.clear();
        self.view.blit(&self.point, Wrapping::Wrap);
        self.view.display_render().unwrap();
    }
}

fn main() {
    let mut game = Game::new();

    game.main_loop(FPS);
}
