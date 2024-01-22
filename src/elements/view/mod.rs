//! This module is home to the [`View`] struct, which handles the printing of pixels to an ANSI standard text output
use crate::utils as crate_utils;
use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

mod pixel;
mod scale_to_fit;
pub mod utils;
mod view_element;
mod wrapping;

#[allow(deprecated)]
pub use pixel::{
    colchar::{ColChar, Colour, Modifier},
    vec2d::Vec2D,
    Pixel, Point,
};
pub use scale_to_fit::ScaleFitView;
pub use view_element::ViewElement;
pub use wrapping::Wrapping;

/// The View struct is the canvas on which you will print all of your `ViewElement`s. In normal use, you would clear the View, `blit` all your `ViewElement`s to it and then render. The following example demonstrates a piece of code that will render a View of width 9 and height 3, with a single Pixel in the middle
/// ```
/// use gemini_engine::elements::{view::{Wrapping, ColChar}, View, Pixel, Vec2D};
///
/// let mut view = View::new(9, 3, ColChar::BACKGROUND);
/// let pixel = Pixel::new(view.center(), ColChar::SOLID);
///
/// view.blit(&pixel, Wrapping::Panic);
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
    /// If true, [`View.display_render`] will block until the console window is resized to fit the `View`
    pub block_until_resized: bool,
    pixels: Vec<ColChar>,
}

impl View {
    /// Create a new `View` using [`width`](View::width), [`height`](View::height) and [`background_char`](View::background_char) parameters
    #[must_use]
    pub fn new(width: usize, height: usize, background_char: ColChar) -> Self {
        let mut view = Self {
            width,
            height,
            background_char,
            coord_numbers_in_render: false,
            block_until_resized: false,
            pixels: Vec::with_capacity(width * height),
        };
        view.clear();

        view
    }

    /// Return the `View` with its [`coord_numbers_in_render`](View::coord_numbers_in_render) field set to the chosen value. Consumes the original `View`
    #[must_use]
    pub const fn with_coord_numbers(mut self, coord_numbers_in_render: bool) -> Self {
        self.coord_numbers_in_render = coord_numbers_in_render;
        self
    }

    /// Return the `View` with its [`block_until_resized`](View::block_until_resized) field set to the chosen value. Consumes the original `View`
    #[must_use]
    pub const fn with_block_until_resized(mut self, block_until_resized: bool) -> Self {
        self.block_until_resized = block_until_resized;
        self
    }

    /// Return the width and height of the `View` as a [`Vec2D`]
    #[must_use]
    pub const fn size(&self) -> Vec2D {
        Vec2D::new(self.width as isize, self.height as isize)
    }

    /// Return [`Vec2D`] coordinates of the centre of the `View`
    #[must_use]
    pub fn center(&self) -> Vec2D {
        self.size() / 2
    }

    /// Clear the `View` of all pixels
    pub fn clear(&mut self) {
        self.pixels = vec![self.background_char; self.width * self.height];
    }

    /// Plot a pixel to the `View`. Accepts a [`Vec2D`] (the position of the pixel), [`ColChar`] (what the pixel should look like/what colour it should be), and a [`Wrapping`] enum variant (Please see the [Wrapping] documentation for more info)
    pub fn plot(&mut self, pos: Vec2D, c: ColChar, wrapping: Wrapping) {
        if let Some(wrapped_pos) = wrapping.handle_bounds(pos, self.size()) {
            self.pixels[self.width * wrapped_pos.y.unsigned_abs() + (wrapped_pos.x.unsigned_abs())] = c;
        }
    }

    /// Blit a struct implementing [`ViewElement`] to the `View`
    pub fn blit(&mut self, element: &impl ViewElement, wrapping: Wrapping) {
        for pixel in element.active_pixels() {
            self.plot(pixel.pos, pixel.fill_char, wrapping);
        }
    }

    /// Blit a struct implementing [`ViewElement`] to the `View` with a doubled width. Blitting a `Pixel` at `Vec2D(5,3)`, for example, will result in a blit at `Vec2D(10,3)` and `Vec2D(11,3)` being plotted to. Useful when you want to work with more square pixels, as single text characters are much taller than they are wide
    pub fn blit_double_width(&mut self, element: &impl ViewElement, wrapping: Wrapping) {
        for pixel in element.active_pixels() {
            let pos = pixel.pos * Vec2D::new(2, 1);
            self.plot(pos, pixel.fill_char, wrapping);
            self.plot(pos + Vec2D::new(1, 0), pixel.fill_char, wrapping);
        }
    }

    /// Display the `View`. `View` implements the `Display` trait and so can be rendered in many ways (such as `println!("{view}");`), but this is intended to be the fastest way possible.
    ///
    /// # Errors
    /// Returns the `Result` from writing to `io::stdout().lock()`. You can ignore it with `let _ = ...` most of the time
    pub fn display_render(&self) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        if self.block_until_resized {
            let view_size = self.size();
            crate_utils::block_until_resized(view_size);
        }

        write!(stdout, "{self}")
    }
}

impl Display for View {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let _ = crate::utils::prepare_terminal(f);
        f.write_str("\x1b[H\x1b[J")?;
        if self.coord_numbers_in_render {
            let nums: String = (0..self.width)
                .map(|i| i.to_string().chars().last().unwrap_or(' '))
                .collect();
            writeln!(f, " {nums}")?;
        }
        for y in 0..self.height {
            if self.coord_numbers_in_render {
                let num = y.to_string().chars().last().unwrap_or(' ');
                write!(f, "{num}")?;
            }

            let row = &self.pixels[self.width * y..self.width * (y + 1)];

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
