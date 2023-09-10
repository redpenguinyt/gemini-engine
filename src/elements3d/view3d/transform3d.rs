use super::Vec3D;

/// The `Transform3D` struct is used to manipulate the position of objects in 3D space
pub struct Transform3D {
    /// The position of the object in 3D space
    pub location: Vec3D,
    /// The rotation of the object, applied in radians
    pub rotation: Vec3D,
    /// The object's scale
    pub scale: Vec3D,
}

impl Transform3D {
    pub const ZERO: Self = Self::new(Vec3D::ZERO, Vec3D::ZERO, Vec3D::ONE);

    pub const fn new(location: Vec3D, rotation: Vec3D, scale: Vec3D) -> Self {
        Self {
            location,
            rotation,
            scale,
        }
    }

    pub fn apply_to_vec3d(&self, value: Vec3D) -> Vec3D {
        let mut value = value * self.scale;
        value.rotate(self.rotation);
        value += self.location;

        value
    }
}
