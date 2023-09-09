use std::{
    cmp::PartialEq,
    fmt::{Display, Result},
    ops::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign},
};

/// Raw Vector2 type
#[derive(Debug, Copy, PartialEq)]
pub struct Vector2<T: Clone> {
    pub x: T,
    pub y: T,
}

impl<T: Clone> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }

    pub fn as_tuple(&self) -> (T, T) {
        (self.x.clone(), self.y.clone())
    }
}

impl<T: Clone> Clone for Vector2<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: Clone + Add<Output = T>> Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Clone + AddAssign> AddAssign<Vector2<T>> for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Clone + Sub<Output = T>> Sub<Vector2<T>> for Vector2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Clone + SubAssign> SubAssign<Vector2<T>> for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Clone> From<(T, T)> for Vector2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

/// A pair of `isize` used for coordinates, size or direction on a 2D plane
pub type Vec2D = Vector2<isize>;

impl Vec2D {
    /// A Vec2D of (0,0)
    pub const ZERO: Vec2D = Vec2D::new(0, 0);
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

impl Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Vec2D({}, {})", self.x, self.y)
    }
}

impl RemAssign for Vec2D {
    fn rem_assign(&mut self, rhs: Self) {
        self.x = self.x.rem_euclid(rhs.x);
        self.y = self.y.rem_euclid(rhs.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vec2() {
        assert_eq!(
            Vector2::new(15, -3),
            Vector2::new(13, 4) + Vector2::new(2, -7)
        );
    }

    #[test]
    fn subtract_vec2() {
        assert_eq!(
            Vector2::new(2, -10),
            Vector2::new(17, 4) - Vector2::new(15, 14)
        );
    }

    #[test]
    fn rem_vec2_over() {
        assert_eq!(
            Vector2::new(4, 1),
            Vector2::new(9, 11) % Vector2::new(5, 10)
        )
    }

    #[test]
    fn rem_vec2_under() {
        assert_eq!(
            Vector2::new(4, 1),
            Vector2::new(-1, -109) % Vector2::new(5, 10)
        )
    }

    #[test]
    fn eq_vec2_both() {
        assert_eq!(Vector2::new(5, 4), Vector2::new(5, 4))
    }

    #[test]
    fn eq_vec2_only_one() {
        assert_ne!(Vector2::new(5, 2), Vector2::new(5, 4))
    }

    #[test]
    fn eq_vec2_neither() {
        assert_ne!(Vector2::new(17, 2), Vector2::new(5, 4))
    }
}
