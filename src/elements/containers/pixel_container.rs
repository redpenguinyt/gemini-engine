use crate::elements::{
    view::{utils, ColChar, ViewElement},
    Pixel, Vec2D,
};

/// A `PixelContainer` only has a [`pixels`](PixelContainer::pixels) property, which gets returned directly to the View during blit
#[derive(Debug, Clone)]
pub struct PixelContainer {
    /// This is the value that gets returned by [`active_pixels()`](ViewElement::active_pixels)
    pub pixels: Vec<Pixel>,
}

impl PixelContainer {
    /// Create a new, empty `PixelContainer`
    pub const fn new() -> Self {
        Self { pixels: vec![] }
    }

    /// Add a single pixel to the `PixelContainer`
    pub fn push(&mut self, pixel: Pixel) {
        self.pixels.push(pixel);
    }

    /// Moves all the pixels into the `PixelContainer`, leaving the input empty.
    pub fn append(&mut self, pixels: &mut Vec<Pixel>) {
        self.pixels.append(pixels);
    }

    /// Append vector of coordinates and a single [`ColChar`] for all of them.
    pub fn append_points(&mut self, points: Vec<Vec2D>, fill_char: ColChar) {
        self.append(&mut utils::points_to_pixels(points, fill_char));
    }

    /// Plot a pixel to the PixelContainer
    pub fn plot(&mut self, pos: Vec2D, c: ColChar) {
        self.push(Pixel::new(pos, c))
    }

    /// Blit a [`ViewElement`] to the PixelContainer.
    pub fn blit<E: ViewElement>(&mut self, element: &E) {
        let mut active_pixels = element.active_pixels();

        self.append(&mut active_pixels);
    }
}

impl From<&[Pixel]> for PixelContainer {
    fn from(pixels: &[Pixel]) -> Self {
        Self {
            pixels: pixels.to_vec(),
        }
    }
}

impl From<&[(Vec2D, ColChar)]> for PixelContainer {
    fn from(pixels: &[(Vec2D, ColChar)]) -> Self {
        Self {
            pixels: pixels.iter().map(|x| Pixel::from(*x)).collect(),
        }
    }
}

impl From<(&[Vec2D], ColChar)> for PixelContainer {
    fn from(value: (&[Vec2D], ColChar)) -> Self {
        Self {
            pixels: value
                .0
                .iter()
                .map(|pos| Pixel::new(*pos, value.1))
                .collect(),
        }
    }
}

impl ViewElement for PixelContainer {
    fn active_pixels(&self) -> Vec<Pixel> {
        self.pixels.clone()
    }
}
