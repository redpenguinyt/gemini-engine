// this absolute mess of code makes things run 1-2% faster so i'm keeping it

use super::Vec3D;

/// Enum to indicate a 3D axis
enum SpatialAxis {
    X,
    Y,
    Z,
}

impl SpatialAxis {
    /// Returns the two axes on the plane perpendicular to the SpatialAxis' variation
    pub fn get_perpendicular_plane<'a>(&self, value: &'a mut Vec3D) -> (&'a mut f64, &'a mut f64) {
        match self {
            SpatialAxis::X => (&mut value.y, &mut value.z),
            SpatialAxis::Y => (&mut value.x, &mut value.z),
            SpatialAxis::Z => (&mut value.x, &mut value.y),
        }
    }
}

/// One axis of cached rotation
struct CachedRotation {
    s: f64,
    c: f64,
}

impl CachedRotation {
    pub fn new(r: f64) -> CachedRotation {
        CachedRotation {
            s: r.sin(),
            c: r.cos(),
        }
    }

    pub fn rotate_one_axis(&self, value: Vec3D, axis: SpatialAxis) -> Vec3D {
        let mut translation = value;
        let (x, y) = axis.get_perpendicular_plane(&mut translation);

        (*x, *y) = (*x * self.c - *y * self.s, *x * self.s + *y * self.c);

        translation
    }
}

/// Accepts a rotation and can be applied to multiple vertices, but only calls all sin functions once
pub struct CachedRotation3D {
    x: CachedRotation,
    y: CachedRotation,
    z: CachedRotation,
}

impl CachedRotation3D {
    pub fn new(rot: Vec3D) -> CachedRotation3D {
        CachedRotation3D {
            x: CachedRotation::new(rot.x),
            y: CachedRotation::new(rot.y),
            z: CachedRotation::new(rot.z),
        }
    }

    #[allow(clippy::let_and_return)]
    pub fn rotate(&self, rhs: Vec3D) -> Vec3D {
        let ry = self.y.rotate_one_axis(rhs, SpatialAxis::Y);
        let rx = self.x.rotate_one_axis(ry, SpatialAxis::X);
        let rz = self.z.rotate_one_axis(rx, SpatialAxis::Z);

        rz
    }
}
