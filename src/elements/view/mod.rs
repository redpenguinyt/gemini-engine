use std::{
    fmt::{self, Display},
    io::{self, Write},
};
pub mod colchar;
mod point;
pub mod utils;
mod vec2d;
pub use colchar::{ColChar, Colour, Modifier};
pub use point::Point;
pub use utils::Wrapping;
pub use vec2d::Vec2D;

/// The View struct is the canvas on which you will print all of your ViewElements. In normal use, you would clear the View, `blit` all your ViewElements to it and then render. The following example demonstrates a piece of code that will render a View of width 9 and height 3, with a single Point in the middle
/// ```
/// use gemini_engine::elements::{view::{Wrapping, ColChar}, View, Point, Vec2D};
///
/// let mut view = View::new(9, 3, ColChar::BACKGROUND);
/// let point = Point::new(Vec2D::new(4,1), ColChar::SOLID);
///
/// view.blit(&point, Wrapping::Panic);
///
/// view.display_render().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct View {
    /// The width of the View
    pub width: usize,
    /// The height of the View
    pub height: usize,
    /// The character that the View will be filled with by default on clear
    pub background_char: ColChar,
    /// A boolean determining whether the render should contain numbers on the top and left signifying the corresponding pixels' X/Y value values
    pub coord_numbers_in_render: bool,
    pixels: Vec<ColChar>,
}

impl View {
    /// Create a new `View` using [`width`](View::width), [`height`](View::height) and [`background_char`](View::background_char) parameters
    pub fn new(width: usize, height: usize, background_char: ColChar) -> View {
        let mut view = View {
            width,
            height,
            background_char,
            coord_numbers_in_render: false,
            pixels: Vec::with_capacity(width * height),
        };
        view.clear();

        view
    }

    /// Return the size of the [`View`] as a [`Vec2D`]
    pub fn size(&self) -> Vec2D {
        Vec2D::new(self.width as isize, self.height as isize)
    }

    /// Return a [`Vec2D`] representing the centre of the `View`
    pub fn center(&self) -> Vec2D {
        self.size() / 2
    }

    /// Clear the `View` of all pixels
    pub fn clear(&mut self) {
        self.pixels = vec![self.background_char; self.width * self.height]
    }

    /// Plot a pixel to the `View`. Accepts a [`Vec2D`] (the position of the pixel), [`ColChar`] (what the pixel should look like/what colour it should be), and a [`Wrapping`] enum variant (Please see the [Wrapping] documentation for more info)
    pub fn plot(&mut self, pos: Vec2D, c: ColChar, wrapping: Wrapping) {
        let mut pos = pos;
        let in_bounds_pos = pos % self.size();

        match wrapping {
            Wrapping::Wrap => pos = in_bounds_pos,
            Wrapping::Ignore => {
                if pos != in_bounds_pos {
                    return;
                }
            }
            Wrapping::Panic => {
                if pos != in_bounds_pos {
                    panic!("{} is out of bounds", pos);
                }
            }
        }

        self.pixels[self.width * (pos.y as usize) + (pos.x as usize)] = c;
    }

    /// Blit a struct implementing [`ViewElement`] to the `View`
    pub fn blit(&mut self, element: &impl ViewElement, wrapping: Wrapping) {
        for point in element.active_pixels() {
            self.plot(point.pos, point.fill_char, wrapping);
        }
    }

    /// Display the `View`. `View` implements the `Display` trait and so can be rendered in many ways (such as `println!("{view}");`), but this is intended to be the fastest way possible.
    pub fn display_render(&self) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        write!(stdout, "{self}")
    }
}

impl Display for View {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = utils::prepare_terminal(f);
        f.write_str("\x1b[H\x1b[J")?;
        if self.coord_numbers_in_render {
            let nums: String = (0..self.width)
                .map(|i| i.to_string().chars().last().unwrap_or(' '))
                .collect();
            writeln!(f, " {}", nums).unwrap();
        }
        for y in 0..self.height {
            let row = &self.pixels[self.width * y..self.width * (y + 1)];
            if self.coord_numbers_in_render {
                let num = y.to_string().chars().last().unwrap_or(' ');
                write!(f, "{num}").unwrap();
            }
            row[0].display_with_prev_and_next(f, None, Some(row[1].modifier))?;
            for x in 1..(row.len() - 1) {
                row[x].display_with_prev_and_next(
                    f,
                    Some(row[x - 1].modifier),
                    Some(row[x + 1].modifier),
                )?;
            }
            row[row.len() - 1].display_with_prev_and_next(
                f,
                Some(row[row.len() - 2].modifier),
                None,
            )?;
            f.write_str("\r\n")?;
        }
        f.write_str("\x1b[J")?;

        Ok(())
    }
}

/// `ViewElement` is a trait that must be implemented by any element that can be blitted to a [`View`]
pub trait ViewElement {
    /// Return a vector of every coordinate where a pixel should be placed and its respective [`ColChar`]. If your whole object is a solid colour, consider using [`utils::points_to_pixels()`] which will add the same [`ColChar`] to every point and can then be used as this function's output
    fn active_pixels(&self) -> Vec<Point>;
}
