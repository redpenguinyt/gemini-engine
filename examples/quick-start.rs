//! The quick start example
use gemini_engine::elements::{
    view::{ColChar, View, Wrapping},
    Pixel, Vec2D,
};
use gemini_engine::gameloop;

const FPS: f32 = 30.0;

fn main() {
    let mut view = View::new(40, 8, ColChar::BACKGROUND);
    let mut pixel = Pixel::new(Vec2D { x: 10, y: 5 }, ColChar::SOLID);

    loop {
        view.clear();

        pixel.pos.x += 1;

        view.blit(&pixel, Wrapping::Wrap);
        let _ = view.display_render();

        let _ = gameloop::sleep_fps(FPS, None);
    }
}
