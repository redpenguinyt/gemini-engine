use std::time::Instant;
mod elements; use elements::{Point, Line, Box, View, Vec2D};
mod gameloop;

/// Missing from gemini-rust:
/// input
/// cameras
/// layers
/// colours
/// visibility
/// parenting (redo to function as node tree)
/// somehow manage pixels out of bounds (maybe on a per-object basis?)
/// collisions (probably as a separate object but able to accept ViewElement as input to create the hitbox)
/// polygon element
/// image element
/// get entities at location (requires parenting)

const FPS: u32 = 20;
const FILL_CHAR: char = '█';
const EMPTY_CHAR: char = '░';

fn main() {
    let mut view = View::new(30, 10, EMPTY_CHAR);

    let mut point_pos = Vec2D::from((5,9));
    let mut line1_direction = -1;

    let point1 = Point::new(
        point_pos + Vec2D::new(2, -8),
        FILL_CHAR,
    );

    let mut line1 = Line::new(
        Vec2D::new(2, 8),
        Vec2D::new(28, 7),
        FILL_CHAR,
    );

    let box1 = Box::new(
        Vec2D { x: 15, y: 1 },
        Vec2D { x: 11, y: 3 },
        FILL_CHAR,
    );

    loop { // Begin game loop
        let now = Instant::now();
        view.clear();

        point_pos.x += 1;
        point_pos %= Vec2D::from(&view);

        line1.pos1.y += line1_direction;
        line1.pos0.y = 10 - line1.pos1.y;
        if line1.pos1.y > 7 {
            line1_direction = -1;
        } else if line1.pos1.y < 3 {
            line1_direction = 1;
        }

        view.plot(point_pos, FILL_CHAR);

        view.blit(&point1);
        view.blit(&line1);
        view.blit(&box1);

        view.render();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?} microseconds", elapsed.as_micros());
        println!("Point position: {point_pos}");

        gameloop::sleep_fps(FPS, Some(elapsed)); // not making use of frame_skip as this particular View is very simple and unlikely to exceed frame duraction
    }
}