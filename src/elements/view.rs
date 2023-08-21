use std::usize;
mod colchar;
pub mod utils;
mod vec2d;
pub use colchar::{ColChar, Modifier};
pub use utils::Wrapping;
pub use vec2d::Vec2D;

/// The main struct for housing your view. Blit ViewElements to the View for them to appear on the scene
pub struct View {
    pub width: usize,
    pub height: usize,
    pub background_char: ColChar,
    pixels: Vec<ColChar>,
    terminal_prepared: bool,
}

impl From<&View> for Vec2D {
    fn from(value: &View) -> Self {
        Vec2D {
            x: isize::try_from(value.width).expect("Failed to convert View.width to isize"),
            y: isize::try_from(value.height).expect("Failed to convert View.height to isize"),
        }
    }
}

impl View {
    pub fn new(width: usize, height: usize, background_char: ColChar) -> View {
        let mut view = View {
            width,
            height,
            background_char,
            pixels: Vec::new(),
            terminal_prepared: false,
        };

        view.clear();
        view.prepare_terminal();

        view
    }

    fn prepare_terminal(&mut self) {
        if !self.terminal_prepared {
            let rows = termsize::get().unwrap().rows;
            let rows_us = usize::try_from(rows).expect("u16 couldnt convert to usize");
            println!(
                "{}",
                vec!['\n'; rows_us].iter().cloned().collect::<String>()
            );
        }
        self.terminal_prepared = true
    }

    pub fn clear(&mut self) {
        self.pixels = vec![self.background_char; self.width * self.height]
    }

    pub fn plot(&mut self, pos: Vec2D, c: ColChar, wrapping: Wrapping) {
        let mut pos = pos;
        let in_bounds_pos = pos.clone() % (Vec2D::from(&*self));

        match wrapping {
            Wrapping::Wrap => pos = in_bounds_pos,
            Wrapping::Ignore => {
                if pos.x < 0 || pos.y < 0 || pos != in_bounds_pos {
                    return;
                }
            }
            Wrapping::Panic => {
                if pos.x < 0 || pos.y < 0 || pos != in_bounds_pos {
                    panic!("{} is not within the view's boundaries", pos);
                }
            }
        }

        let ux = pos.x as usize;
        let uy = pos.y as usize;

        self.pixels[self.width * uy + ux] = c;
    }

    /// Blit a ViewElement to the screen. This is usually done before rendering.
    pub fn blit<T: ViewElement>(&mut self, element: &T, wrapping: Wrapping) {
        let active_pixels = element.active_pixels();

        for (pixel, fill_char) in active_pixels {
            self.plot(pixel, fill_char, wrapping);
        }
    }

    pub fn render(&self) {
        print!("\x1b[H\x1b[J");
        for y in 0..self.height {
            let row_pixels = self.pixels[self.width * y..self.width * (y + 1)].iter();

            let mut row = String::new();
            for pixel in row_pixels {
                row.push_str(pixel.render().as_str());
            }

            println!("{row}");
        }
        println!("\x1b[J");
    }
}

/// ViewElement is a trait that must be implemented by any element that can be blitted to a View
pub trait ViewElement {
    /// Return an array of every pixel where the object exists and the character that should be placed at that pixel
    fn active_pixels(&self) -> Vec<(Vec2D, ColChar)>;
}
