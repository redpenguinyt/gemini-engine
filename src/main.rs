use std::{usize, thread::sleep, time::Duration};
mod vec2d;
use vec2d::Vec2D;

const FPS: f64 = 20.0;

const FILL_CHAR: char = '█';
const EMPTY_CHAR: char = '░';

impl From<&View> for Vec2D {
    fn from(value: &View) -> Self {
        Vec2D {
            x: isize::try_from(value.width).expect("Failed to convert View.width to isize"),
            y: isize::try_from(value.height).expect("Failed to convert View.height to isize")
        }
    }
}

struct View {
    width: usize,
    height: usize,
    pixels: Vec<char>,
    empty_char: char,
}

impl View {
    fn new(width: usize, height: usize, empty_char: char) -> View {
        let mut view = View {
            width, height,
            pixels: Vec::new(),
            empty_char,
        };

        view.clear();

        view
    }

    fn clear(&mut self) {
        self.pixels = vec![self.empty_char; self.width * self.height]
    }

    fn plot(&mut self, pos: &Vec2D, c: char) {
        self.pixels[pos.to_view_position(self.width)] = c;
    }

    fn blit<T:ViewElement>(&mut self, element: &T) {
        let active_pixels = element.active_pixels();

        for (pixel, fill_char) in active_pixels {
            self.plot(&pixel, fill_char);
        }
    }

    fn render(&self) {
        print!("\x1b[H\x1b[J");
        for y in 0..self.height {
            let row: String = self.pixels[self.width * y..self.width * (y+1)].iter().collect();

            println!("{row}");
        }
        println!("\x1b[J");
    }
}

trait ViewElement {
    fn active_pixels(&self) -> Vec<(&Vec2D, char)>;
}

struct Point {
    position: Vec2D,
    fill_char: char
}

impl ViewElement for Point {
    fn active_pixels(&self) -> Vec<(&Vec2D, char)> {
        return Vec::from([(&self.position, self.fill_char)]);
    }
}

fn main() {
    let mut view = View::new(30, 10, EMPTY_CHAR);
    let mut point_pos = Vec2D::from((5,5));

    let point = Point {
        position: point_pos - Vec2D {x: -1, y: 3},
        fill_char: FILL_CHAR
    };

    loop {
        view.clear();

        point_pos.x += 1;
        point_pos %= Vec2D::from(&view);

        view.plot(&point_pos, FILL_CHAR);

        view.blit(&point);

        view.render();
        sleep(Duration::from_secs_f64(1.0/FPS));
    }
}