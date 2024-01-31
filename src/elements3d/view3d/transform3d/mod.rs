use std::ops::{Mul, MulAssign, Neg};
mod vec3d;
pub use vec3d::Vec3D;
mod fast_rotate;
use fast_rotate::CachedRotation3D;

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

impl Default for Transform3D {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Transform3D {
    /// The default transform - no translation, no rotation and 1x scaling
    pub const DEFAULT: Self = Self::new_trs(Vec3D::ZERO, Vec3D::ZERO, Vec3D::ONE);

    /// Create a `Transform3D` with chosen translation, rotation and scale
    #[must_use]
    pub const fn new_trs(translation: Vec3D, rotation: Vec3D, scale: Vec3D) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    /// Create a `Transform3D` with chosen translation and rotation
    #[must_use]
    pub const fn new_tr(translation: Vec3D, rotation: Vec3D) -> Self {
        Self {
            translation,
            rotation,
            scale: Vec3D::ONE,
        }
    }

    /// Create a `Transform3D` with chosen translation
    #[must_use]
    pub const fn new_t(translation: Vec3D) -> Self {
        Self {
            translation,
            rotation: Vec3D::ZERO,
            scale: Vec3D::ONE,
        }
    }

    /// Create a `Transform3D` with chosen rotation
    #[must_use]
    pub const fn new_r(rotation: Vec3D) -> Self {
        Self {
            translation: Vec3D::ZERO,
            rotation,
            scale: Vec3D::ONE,
        }
    }

    /// Apply the transform to a slice of vertices
    #[allow(clippy::let_and_return)]
    #[must_use]
    pub fn apply_to(&self, vertices: &[Vec3D]) -> Vec<Vec3D> {
        let rotation = CachedRotation3D::new(self.rotation);

        vertices
            .iter()
            .map(|v| {
                let rhs = *v;
                let rhs = rhs * self.scale;
                let rhs = rotation.rotate(rhs);
                let rhs = rhs + self.translation;

                rhs
            })
            .collect()
    }

    /// Apply the transform to a slice of vertices as if it is a viewport transform
    #[allow(clippy::let_and_return)]
    #[must_use]
    pub(crate) fn apply_viewport_transform(&self, vertices: &[Vec3D]) -> Vec<Vec3D> {
        let rotation = CachedRotation3D::new(-self.rotation);

        vertices
            .iter()
            .map(|v| { // Don't do scale at all
                let rhs = *v;
                let rhs = rhs - self.translation; // Translate before rotating
                let rhs = (rotation).rotate(rhs);

                rhs
            })
            .collect()
    }

    /// Rotate the given [`Vec3D`] using the `Transform3D`'s rotation field
    #[must_use]
    pub fn rotate(&self, value: Vec3D) -> Vec3D {
        let rotation = CachedRotation3D::new(self.rotation);

        rotation.rotate(value)
    }
}

impl Neg for Transform3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new_trs(-self.translation, -self.rotation, self.scale)
    }
}

impl Mul<Self> for Transform3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_trs(
            self.translation + rhs.translation,
            self.rotation + rhs.rotation,
            self.scale * rhs.scale,
        )
    }
}

impl MulAssign<Self> for Transform3D {
    fn mul_assign(&mut self, rhs: Self) {
        self.translation += rhs.translation;
        self.rotation += rhs.rotation;
        self.scale *= rhs.scale;
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
