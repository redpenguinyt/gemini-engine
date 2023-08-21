use std::{io::Error, usize};
pub mod colchar;
pub mod utils;
pub mod vec2d;
pub use colchar::{ColChar, Modifier};
pub use utils::Wrapping;
pub use vec2d::Vec2D;

/// The View struct is the canvas on which you will print all of your ViewElements. In normal use, you would clear the View, `blit` all your ViewElements to it and then render. The following example demonstrates a piece of code that will render a View of width 9 and height 3, with a single Point in the middle
/// ```
/// use gemini::elements::{view::{Wrapping, ColChar}, View, Point, Vec2D};
///
/// fn main() {
///     let mut view = View::new(9, 3, ColChar::BACKGROUND);
///     let point = Point::new(Vec2D::new(4,1), ColChar::SOLID);
///
///     view.blit(&point, Wrapping::Panic);
///
///     view.render();
/// }
/// ```
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
        let _ = view.prepare_terminal(); // TODO: handle potential error somehow

        view
    }

    fn prepare_terminal(&mut self) -> Result<(), Error> {
        if !self.terminal_prepared {
            let rows = termsize::get()
                .ok_or(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Couldnt get termsize",
                ))?
                .rows;
            let rows_us = usize::try_from(rows).expect("u16 couldnt convert to usize");
            println!(
                "{}",
                vec!['\n'; rows_us].iter().cloned().collect::<String>()
            );
        }
        self.terminal_prepared = true;

        Ok(())
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
