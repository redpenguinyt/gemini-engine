use std::ops::Mul;
mod vec3d;
pub use vec3d::Vec3D;

enum SpatialAxis {
    X,
    Y,
    Z,
}

/// The `Transform3D` struct is used to manipulate the position of objects in 3D space
#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    /// The position of the object in 3D space
    pub translation: Vec3D,
    /// The rotation of the object, applied in radians
    pub rotation: Vec3D,
    /// The object's scale
    pub scale: Vec3D,
}

impl Transform3D {
    /// The default transform - no translation, no rotation and 1x scaling
    pub const DEFAULT: Self = Self::new_trs(Vec3D::ZERO, Vec3D::ZERO, Vec3D::ONE);

    /// Create a Transform3D with chosen translation, rotation and scale
    pub const fn new_trs(translation: Vec3D, rotation: Vec3D, scale: Vec3D) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    /// Create a Transform3D with chosen translation and rotation
    pub const fn new_tr(translation: Vec3D, rotation: Vec3D) -> Self {
        Self {
            translation,
            rotation,
            scale: Vec3D::ONE,
        }
    }

    /// Create a Transform3D with chosen translation
    pub const fn new_t(translation: Vec3D) -> Self {
        Self {
            translation,
            rotation: Vec3D::ZERO,
            scale: Vec3D::ONE,
        }
    }

    /// Create a Transform3D with chosen rotation
    pub const fn new_r(rotation: Vec3D) -> Self {
        Self {
            translation: Vec3D::ZERO,
            rotation,
            scale: Vec3D::ONE,
        }
    }

    /// Rotate the [`Vec3D`] on one axis
    fn rotate_one_axis(translation: Vec3D, axis: SpatialAxis, single_rotation: f64) -> Vec3D {
        if single_rotation == 0.0 {
            return translation;
        }
        let mut translation = translation;
        let (x, y) = match axis {
            SpatialAxis::X => (&mut translation.y, &mut translation.z),
            SpatialAxis::Y => (&mut translation.x, &mut translation.z),
            SpatialAxis::Z => (&mut translation.x, &mut translation.y),
        };

        let s = single_rotation.sin();
        let c = single_rotation.cos();
        (*x, *y) = (*x * c - *y * s, *x * s + *y * c);

        translation
    }

    /// Rotate the given [`Vec3D`] using the `Transform3D`'s rotation field
    #[allow(clippy::let_and_return)]
    pub fn rotate(&self, value: Vec3D) -> Vec3D {
        let ry = Self::rotate_one_axis(value, SpatialAxis::Y, self.rotation.y);
        let rx = Self::rotate_one_axis(ry, SpatialAxis::X, self.rotation.x);
        let rz = Self::rotate_one_axis(rx, SpatialAxis::Z, self.rotation.z);

        rz
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Mul<Transform3D> for Transform3D {
    type Output = Transform3D;

    fn mul(self, rhs: Transform3D) -> Self::Output {
        Self::new_trs(
            self.translation + rhs.translation,
            self.rotation + rhs.rotation,
            self.scale * rhs.scale,
        )
    }
}

impl Mul<Vec3D> for Transform3D {
    type Output = Vec3D;

    /// Apply the transform to the `Vec3D`
    #[allow(clippy::let_and_return)]
    fn mul(self, rhs: Vec3D) -> Self::Output {
        let rhs = rhs * self.scale;
        let rhs = self.rotate(rhs);
        let rhs = rhs + self.translation;

        rhs
    }
}
