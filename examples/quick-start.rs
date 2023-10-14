use gemini_engine::elements::{
    view::{ColChar, View, Wrapping},
    Pixel, Vec2D,
};
use gemini_engine::gameloop;

const FPS: f32 = 30.0;

fn main() {
    let mut view = View::new(40, 8, ColChar::BACKGROUND);
    let mut point = Pixel::new(Vec2D { x: 10, y: 5 }, ColChar::SOLID);

    loop {
        view.clear();

        point.pos.x += 1;

        view.blit(&point, Wrapping::Wrap);
        view.display_render().unwrap();

        gameloop::sleep_fps(FPS, None);
    }
}
