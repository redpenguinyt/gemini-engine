use std::usize;
mod vec2d; pub use vec2d::Vec2D;

/// The main struct for housing your view. Blit ViewElements to the View for them to appear on the scene
pub struct View {
    pub width: usize,
    pub height: usize,
    pub empty_char: char,
    pixels: Vec<char>,
    terminal_prepared: bool
}

impl From<&View> for Vec2D {
    fn from(value: &View) -> Self {
        Vec2D {
            x: isize::try_from(value.width).expect("Failed to convert View.width to isize"),
            y: isize::try_from(value.height).expect("Failed to convert View.height to isize")
        }
    }
}

impl View {
    pub fn new(width: usize, height: usize, empty_char: char) -> View {
        let mut view = View {
            width, height,
            empty_char,
            pixels: Vec::new(),
            terminal_prepared: false
        };

        view.clear();
        view.prepare_terminal();

        view
    }

    fn prepare_terminal(&mut self) {
        if !self.terminal_prepared {
            let rows = termsize::get().unwrap().rows;
            let rows_us = usize::try_from(rows).expect("u16 couldnt convert to usize");
            println!("{}", vec!['\n';rows_us].iter().cloned().collect::<String>());
        }
        self.terminal_prepared = true
    }

    pub fn clear(&mut self) {
        self.pixels = vec![self.empty_char; self.width * self.height]
    }

    pub fn plot(&mut self, pos: Vec2D, c: char) {
        let in_bounds_pos = pos % Vec2D::from(&*self);

        assert_eq!(pos, in_bounds_pos); // TODO: Implement proper error raising here with some error message

        let ux = usize::try_from(pos.x).expect("Failed to convert Vec2D.x to usize");
        let uy = usize::try_from(pos.y).expect("Failed to convert Vec2D.y to usize");

        self.pixels[self.width * uy + ux] = c;
    }

    pub fn blit<T:ViewElement>(&mut self, element: &T) {
        let active_pixels = element.active_pixels();

        for (pixel, fill_char) in active_pixels {
            self.plot(pixel, fill_char);
        }
    }

    pub fn render(&self) {
        print!("\x1b[H\x1b[J");
        for y in 0..self.height {
            let row: String = self.pixels[self.width * y..self.width * (y+1)].iter().collect();

            println!("{row}");
        }
        println!("\x1b[J");
    }
}

/// ViewElement is a trait that must be implemented by any element that can be blitted to a View
pub trait ViewElement {
    /// Return an array of every pixel where the object exists and the character that should be placed at that pixel
    fn active_pixels(&self) -> Vec<(Vec2D, char)>;
}