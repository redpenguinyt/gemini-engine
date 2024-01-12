use crate::elements::Pixel;

/// To write a shader you must have a struct that implements this shader
pub trait CanShade {
    /// This function accepts a pixel and returns the adjusted pixel, as you wish to adjust it
    fn shade(&mut self, pixel: Pixel) -> Pixel;
}