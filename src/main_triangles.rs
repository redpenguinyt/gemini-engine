use std::time::Instant;
mod elements;
use elements::{Triangle, Vec2D, View};
mod gameloop;

const FPS: u32 = 1;
const FILL_CHAR: char = '█';
const EMPTY_CHAR: char = '░';

fn main() {
    let mut view = View::new(405, 110, EMPTY_CHAR);
    let mut frame_skip = false;

    let triangle0 = Triangle::new(
        Vec2D::new(350, 80),
        Vec2D::new(150, 100),
        Vec2D::new(100, 30),
        FILL_CHAR,
    );
    let triangle1 = Triangle::new(
        Vec2D::new(100, 30),
        Vec2D::new(350, 80),
        Vec2D::new(200, 20),
        FILL_CHAR,
    );

    loop {
        // Begin game loop
        let now = Instant::now();
        view.clear();

        if frame_skip {
            frame_skip = false;
        } else {
            view.blit(&triangle0);
            view.blit(&triangle1);

            view.render();
        }
        let elapsed = now.elapsed();
        println!(
            "Elapsed: {:.2?} microseconds | Frame skip: {}",
            elapsed.as_micros(),
            frame_skip
        );

        frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
    }
}
