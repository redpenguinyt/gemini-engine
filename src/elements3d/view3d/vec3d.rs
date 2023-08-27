use super::{super::ViewElement3D, Viewport};
use std::{
    cmp::PartialEq,
    fmt::{Display, Result},
    ops::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign},
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
    pub const ZERO: Self = Vec3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
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

impl Rem<Vec3D> for Vec3D {
    type Output = Vec3D;
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
