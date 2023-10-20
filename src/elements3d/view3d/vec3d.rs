use std::{
    cmp::PartialEq,
    fmt::{Display, Result},
};

/// A point in 3D space, using `f64`s
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3D {
    /// X-coordinate
    pub x: f64,
    /// Y-coordinate
    pub y: f64,
    /// Z-coordinate
    pub z: f64,
}

impl Vec3D {
    impl_vec_single_value_const!(Vec3D, ZERO, 0.0, (x, y, z));
    impl_vec_single_value_const!(Vec3D, ONE, 1.0, (x, y, z));

    impl_vec_core!(Vec3D, f64, (x, y, z));

    /// Return the dot product in combination with another `Vec3D`
    pub fn dot(&self, other: Vec3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the dot product in combination with itself
    pub fn dot_self(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// The length/magnitude of the Vec3D
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross(&self, other: Vec3D) -> Vec3D {
        Vec3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Vec3D({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Into<f64>> From<(T, T, T)> for Vec3D {
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
        }
    }
}

impl_vec_add!(Vec3D, (x, y, z));
impl_vec_sub!(Vec3D, (x, y, z));
impl_vec_neg!(Vec3D, 0.0, (x, y, z));
impl_vec_mul!(Vec3D, (x, y, z));
impl_vec_mul_single!(Vec3D, f64, (x, y, z));
impl_vec_div!(Vec3D, (x, y, z));
impl_vec_div_single!(Vec3D, f64, (x, y, z));
impl_vec_rem!(Vec3D, (x, y, z));
