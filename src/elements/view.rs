use std::{
    fmt::{self, Display, Write},
    io::{self, Error, Write as ioWrite},
    usize,
};
pub mod colchar;
pub mod utils;
pub mod vec2d;
pub use colchar::{ColChar, Colour, Modifier};
pub use utils::Wrapping;
pub use vec2d::Vec2D;

/// The View struct is the canvas on which you will print all of your ViewElements. In normal use, you would clear the View, `blit` all your ViewElements to it and then render. The following example demonstrates a piece of code that will render a View of width 9 and height 3, with a single Point in the middle
/// ```
/// use gemini_engine::elements::{view::{Wrapping, ColChar}, View, Point, Vec2D};
///
/// fn main() {
///     let mut view = View::new(9, 3, ColChar::BACKGROUND);
///     let point = Point::new(Vec2D::new(4,1), ColChar::SOLID);
///
///     view.blit(&point, Wrapping::Panic);
///
///     view.display_render().unwrap();
/// }
/// ```
pub struct View {
    pub width: usize,
    pub height: usize,
    pub background_char: ColChar,
    pixels: Vec<ColChar>,
    terminal_prepared: bool,
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

    pub fn size(&self) -> Vec2D {
        Vec2D::new(self.width as isize, self.height as isize)
    }

    // Return a Vec2D of the centre of the screen
    pub fn center(&self) -> Vec2D {
        Vec2D::new((self.width / 2) as isize, (self.height / 2) as isize)
    }

    pub fn clear(&mut self) {
        self.pixels = vec![self.background_char; self.width * self.height]
    }

    pub fn plot(&mut self, pos: Vec2D, c: ColChar, wrapping: Wrapping) {
        let mut pos = pos;
        let in_bounds_pos = pos.clone() % self.size();

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

        for point in active_pixels {
            self.plot(point.pos, point.fill_char, wrapping);
        }
    }

    /// Display the View. View implements the Display trait so you can display it how you wish, but this is intended to be the fastest way possible
    pub fn display_render(&self) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        write!(stdout, "{self}")
    }
}

impl Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b[H\x1b[J")?;
        for y in 0..self.height {
            let row: Vec<&ColChar> = self.pixels[self.width * y..self.width * (y + 1)]
                .iter()
                .collect();
            write!(f, "{}{}", row[0].modifier, row[0].fill_char)?;
            for i in 1..row.len() {
                let curr_mod = row[i].modifier;
                let prev_mod = match row.get(i - 1) {
                    Some(m) => m.modifier,
                    None => Modifier::None,
                };
                let next_mod = match row.get(i + 1) {
                    Some(m) => m.modifier,
                    None => Modifier::None,
                };

                let modifier = match prev_mod == curr_mod {
                    true => Modifier::None,
                    false => curr_mod,
                };
                let end = match next_mod == curr_mod {
                    true => Modifier::None,
                    false => Modifier::END,
                };

                write!(f, "{}{}{}", modifier, row[i].fill_char, end)?;
            }
            f.write_char('\n')?;
        }
        f.write_str("\x1b[J")?;

        Ok(())
    }
}

/// The `Point` holds a single [`Vec2D`], the coordinates at which it is printed when blit to a [`View`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub pos: Vec2D,
    pub fill_char: ColChar,
}

impl Point {
    pub fn new(pos: Vec2D, fill_char: ColChar) -> Self {
        Self { pos, fill_char }
    }
}

impl From<(Vec2D, ColChar)> for Point {
    fn from(value: (Vec2D, ColChar)) -> Self {
        Self {
            pos: value.0,
            fill_char: value.1,
        }
    }
}

impl ViewElement for Point {
    fn active_pixels(&self) -> Vec<Point> {
        vec![*self]
    }
}

/// ViewElement is a trait that must be implemented by any element that can be blitted to a View
pub trait ViewElement {
    /// Return a vector of every coordinate where a pixel should be placed and its respective [`ColChar`]. If your whole object is a solid colour, consider using [`utils::points_to_pixels()`] which will add the same [`ColChar`] to every point and can then be used as this function's output
    fn active_pixels(&self) -> Vec<Point>;
}
