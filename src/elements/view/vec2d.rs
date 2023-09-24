use std::{
    cmp::PartialEq,
    fmt::{Display, Result},
    ops::{Add, AddAssign, Div, DivAssign, Rem, RemAssign, Sub, SubAssign},
};

/// A pair of `isize` used for coordinates, size or direction on a 2D plane
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2D {
    pub x: isize,
    pub y: isize,
}

impl Vec2D {
    /// A Vec2D of (0,0)
    pub const ZERO: Vec2D = Vec2D::new(0, 0);

    /// Create a new `Vec2D` value from two isize values
    pub const fn new(x: isize, y: isize) -> Self {
        Vec2D { x, y }
    }

    /// Return the `Vec2D` as a tuple
    pub fn as_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

impl Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vec2D> for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vec2D> for Vec2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vec2D> for Vec2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Div<isize> for Vec2D {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<isize> for Vec2D {
    fn div_assign(&mut self, rhs: isize) {
        self.x /= rhs;
        self.y /= rhs;
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

impl Rem for Vec2D {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

impl RemAssign for Vec2D {
    fn rem_assign(&mut self, rhs: Self) {
        self.x = self.x.rem_euclid(rhs.x);
        self.y = self.y.rem_euclid(rhs.y);
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Vec2D({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vec2() {
        assert_eq!(Vec2D::new(15, -3), Vec2D::new(13, 4) + Vec2D::new(2, -7));
    }

    #[test]
    fn subtract_vec2() {
        assert_eq!(Vec2D::new(2, -10), Vec2D::new(17, 4) - Vec2D::new(15, 14));
    }

    #[test]
    fn rem_vec2_over() {
        assert_eq!(Vec2D::new(4, 1), Vec2D::new(9, 11) % Vec2D::new(5, 10))
    }

    #[test]
    fn rem_vec2_under() {
        assert_eq!(Vec2D::new(4, 1), Vec2D::new(-1, -109) % Vec2D::new(5, 10))
    }

    #[test]
    fn eq_vec2_both() {
        assert_eq!(Vec2D::new(5, 4), Vec2D::new(5, 4))
    }

    #[test]
    fn eq_vec2_only_one() {
        assert_ne!(Vec2D::new(5, 2), Vec2D::new(5, 4))
    }

    #[test]
    fn eq_vec2_neither() {
        assert_ne!(Vec2D::new(17, 2), Vec2D::new(5, 4))
    }
}
