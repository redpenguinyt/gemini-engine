use super::{super::ViewElement3D, Viewport};
use std::{
    cmp::PartialEq,
    fmt::{Display, Result},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

/// Helper enum for when you need to choose an axis
pub enum SpatialAxis {
    X,
    Y,
    Z,
}

#[derive(Copy, Debug)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub const ZERO: Self = Vec3D::new(0.0, 0.0, 0.0);
    pub const ONE: Self = Vec3D::new(1.0, 1.0, 1.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn rotate_one_axis(&mut self, axis: SpatialAxis, r: f64) {
        let (x, y) = match axis {
            SpatialAxis::X => (&mut self.y, &mut self.z),
            SpatialAxis::Y => (&mut self.x, &mut self.z),
            SpatialAxis::Z => (&mut self.x, &mut self.y),
        };

        let s = r.sin();
        let c = r.cos();

        (*x, *y) = (*x * c - *y * s, *x * s + *y * c)
    }

    pub fn rotate(&mut self, rotation: Vec3D) {
        self.rotate_one_axis(SpatialAxis::Y, rotation.y);
        self.rotate_one_axis(SpatialAxis::X, rotation.x);
        self.rotate_one_axis(SpatialAxis::Z, rotation.z);
    }

    pub fn global_position<T: ViewElement3D>(&self, viewport: &Viewport, object: &T) -> Vec3D {
        let mut pos = *self;

        pos.rotate(object.get_rotation());
        pos += object.get_pos();

        pos.rotate(viewport.rotation);
        pos += viewport.offset;

        pos
    }
}

impl Clone for Vec3D {
    fn clone(&self) -> Self {
        Vec3D {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Vec3D({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialEq for Vec3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Neg for Vec3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::ZERO - self
    }
}

impl Add<Vec3D> for Vec3D {
    type Output = Vec3D;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vec3D> for Vec3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3D> for Vec3D {
    type Output = Vec3D;
    fn sub(self, rhs: Vec3D) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vec3D> for Vec3D {
    fn sub_assign(&mut self, rhs: Vec3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3D> for Vec3D {
    type Output = Self;

    fn mul(self, rhs: Vec3D) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<Vec3D> for Vec3D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f64> for Vec3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Vec3D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Rem<Vec3D> for Vec3D {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
            z: self.z.rem_euclid(rhs.z),
        }
    }
}

impl RemAssign<Vec3D> for Vec3D {
    fn rem_assign(&mut self, rhs: Vec3D) {
        self.x = self.x.rem_euclid(rhs.x);
        self.y = self.y.rem_euclid(rhs.y);
        self.z = self.z.rem_euclid(rhs.z);
    }
}

impl From<(f64, f64, f64)> for Vec3D {
    fn from(value: (f64, f64, f64)) -> Self {
        Vec3D {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
