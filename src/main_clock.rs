use std::{thread::sleep, time::{Duration, Instant}};
mod elements; use elements::{Line, View, Vec2D};


const FPS: f64 = 20.0;
const FILL_CHAR: char = '█';
const EMPTY_CHAR: char = '░';

fn main() {
	let centre = Vec2D::new(16, 7);
	let mut seconds = 0.0;
    let mut view = View::new(33, 15, EMPTY_CHAR);
    let mut second_hand = Line::new(
        centre, centre,
        FILL_CHAR,
    );

    loop { // Begin game loop
        let now = Instant::now();
        view.clear();

		seconds += 1.0;
		let angle = (seconds * 6.0f32 - 90.0).to_radians();
		second_hand.pos1.x = centre.x + (angle.cos()*12.0).round() as isize;
		second_hand.pos1.y = centre.y + (angle.sin()*6.0).round() as isize;

		view.plot(centre, FILL_CHAR);
        view.blit(&second_hand);

        view.render();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?} microseconds", elapsed.as_micros());

        sleep(Duration::from_secs_f64(1.0/FPS));
    }
}