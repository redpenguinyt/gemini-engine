use std::time::Instant;
mod elements; use elements::{Point, Line, View, Vec2D};
mod gameloop;

const FPS: u32 = 60;
const FILL_CHAR: char = '█';
const EMPTY_CHAR: char = '░';

fn main() {
    let mut view = View::new(405, 110, EMPTY_CHAR);
    let mut frame_skip = false;

    let mut point1 = Point::new(
        Vec2D::new(5,9),
        FILL_CHAR,
    );

    let mut line1 = Line::new(
        Vec2D::ZERO,
        Vec2D::ZERO,
        FILL_CHAR,
    );

    let centre0 = Vec2D::new(100, 30);
    let mut degrees0 = 0.0;
    let centre1 = Vec2D::new(350, 80);
    let mut degrees1 = 0.0;

    loop { // Begin game loop
        let now = Instant::now();
        view.clear();

        point1.pos.x += 1;
        point1.pos %= Vec2D::from(&view);

        degrees0 += 2.0;
		let angle = (degrees0 * 6.0f32 - 90.0).to_radians();
		line1.pos0.x = centre0.x + (angle.cos()*12.0).round() as isize;
		line1.pos0.y = centre0.y + (angle.sin()*6.0).round() as isize;

        degrees1 += 1.0;
		let angle = (degrees1 * 6.0f32 - 90.0).to_radians();
		line1.pos1.x = centre1.x + (angle.cos()*24.0).round() as isize;
		line1.pos1.y = centre1.y + (angle.sin()*12.0).round() as isize;

        if frame_skip {
            frame_skip = false;
        } else {
            view.blit(&point1);

            view.blit(&line1);

            view.blit(&Line::new(centre0, line1.pos0, FILL_CHAR));
            view.blit(&Line::new(centre1, line1.pos1, FILL_CHAR));

            view.render();
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?} microseconds | Frame skip: {}", elapsed.as_micros(), frame_skip);

        frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
    }
}