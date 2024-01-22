use std::{
    cmp::PartialEq,
    fmt::{Display, Result},
};

/// A pair of `isize` used for coordinates, size or direction on a 2D plane
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Vec2D {
    /// X-coordinate
    pub x: isize,
    /// Y-coordinate
    pub y: isize,
}

impl Vec2D {
    impl_vec_single_value_const!(Vec2D, ZERO, 0, (x, y));

    impl_vec_core!(Vec2D, isize, (x, y));

    /// The length/magnitude of the `Vec2D`
    #[must_use]
    pub fn magnitude(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Vec2D({}, {})", self.x, self.y)
    }
}

impl<T: Into<isize>> From<(T, T)> for Vec2D {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}
impl_vec_add!(Vec2D, (x, y));
impl_vec_sub!(Vec2D, (x, y));
impl_vec_neg!(Vec2D, 0, (x, y));
impl_vec_mul!(Vec2D, (x, y));
impl_vec_mul_single!(Vec2D, isize, (x, y));
impl_vec_div!(Vec2D, (x, y));
impl_vec_div_single!(Vec2D, isize, (x, y));
impl_vec_rem!(Vec2D, (x, y));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_vec2_both() {
        assert_eq!(Vec2D::new(5, 4), Vec2D::new(5, 4));
    }

    #[test]
    fn eq_vec2_only_one() {
        assert_ne!(Vec2D::new(5, 2), Vec2D::new(5, 4));
    }

    #[test]
    fn eq_vec2_neither() {
        assert_ne!(Vec2D::new(17, 2), Vec2D::new(5, 4));
    }
}
