use super::Vec3D;
use std::ops::Mul;

/// Helper enum for when you need to choose an axis
pub enum SpatialAxis {
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
            translation: translation,
            rotation: Vec3D::ZERO,
            scale: Vec3D::ONE,
        }
    }

    pub fn rotate_one_axis(translation: Vec3D, axis: SpatialAxis, single_rotation: f64) -> Vec3D {
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

    pub fn rotate(&self, value: Vec3D) -> Vec3D {
        let ry = Self::rotate_one_axis(value, SpatialAxis::Y, self.rotation.y);
        let rx = Self::rotate_one_axis(ry, SpatialAxis::X, self.rotation.x);
        let rz = Self::rotate_one_axis(rx, SpatialAxis::Z, self.rotation.z);

        rz
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

    fn mul(self, rhs: Vec3D) -> Self::Output {
        let scaled = rhs * self.scale;
        let rotated = self.rotate(scaled);
        let translated = rotated + self.translation;

        translated
    }
}
