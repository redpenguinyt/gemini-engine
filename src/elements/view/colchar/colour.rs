use std::ops::{Add, AddAssign, Mul, MulAssign};

fn mul_u8_by_f64(value: u8, rhs: f64) -> u8 {
    (value as f64 * rhs).round() as u8
}

/// A struct to contain colour values. Can be created from RGB, HSV or greyscale values, but is ultimately stored as RGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    /// Create a `Colour` object
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn greyscale(v: u8) -> Self {
        Self::rgb(v, v, v)
    }

    pub fn hsv(h: u8, s: u8, v: u8) -> Self {
        let h = h as f32 / 255.0;
        let s = s as f32 / 255.0;
        let v = v as f32 / 255.0;

        let i = (h * 6.0).floor();
        let f = h * 6.0 - i;
        let p = v * (1.0 - f * s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        let (r, g, b) = [
            (v, t, p),
            (q, v, p),
            (p, v, t),
            (p, q, v),
            (t, p, v),
            (v, p, q),
        ][(i % 6.0).floor() as usize];

        Self::rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}

impl Add for Colour {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::rgb(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::rgb(
            mul_u8_by_f64(self.r, rhs),
            mul_u8_by_f64(self.g, rhs),
            mul_u8_by_f64(self.b, rhs),
        )
    }
}

impl MulAssign<f64> for Colour {
    fn mul_assign(&mut self, rhs: f64) {
        self.r = mul_u8_by_f64(self.r, rhs);
        self.r = mul_u8_by_f64(self.g, rhs);
        self.r = mul_u8_by_f64(self.b, rhs);
    }
}
