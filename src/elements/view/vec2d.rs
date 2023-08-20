use std::{
    cmp::PartialEq,
    fmt::{Display, Result},
    ops::{Add, AddAssign, Rem, RemAssign, Sub},
};

#[derive(Debug, Copy)]
pub struct Vec2D {
    pub x: isize,
    pub y: isize,
}

/// A pair of numbers used for coordinates or object size
impl Vec2D {
    /// A Vec2D of (0,0)
    pub const ZERO: Vec2D = Vec2D::new(0, 0);

    pub const fn new(x: isize, y: isize) -> Self {
        Vec2D { x, y }
    }

    pub fn as_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

impl Clone for Vec2D {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Vec2D({}, {})", self.x, self.y)
    }
}

impl PartialEq for Vec2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add<Vec2D> for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: Vec2D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vec2D> for Vec2D {
    fn add_assign(&mut self, rhs: Vec2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vec2D> for Vec2D {
    type Output = Vec2D;
    fn sub(self, rhs: Vec2D) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Rem<Vec2D> for Vec2D {
    type Output = Vec2D;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

impl RemAssign<Vec2D> for Vec2D {
    fn rem_assign(&mut self, rhs: Vec2D) {
        self.x = self.x.rem_euclid(rhs.x);
        self.y = self.y.rem_euclid(rhs.y);
    }
}

impl From<(isize, isize)> for Vec2D {
    fn from(value: (isize, isize)) -> Self {
        Vec2D {
            x: value.0,
            y: value.1,
        }
    }
}
